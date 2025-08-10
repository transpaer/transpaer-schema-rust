use std::collections::HashSet;

// TODO: should error if strings are not the same.
fn merge_optional_strings(s1: &Option<String>, s2: &Option<String>) -> Option<String> {
    s1.as_ref().or(s2.as_ref()).cloned()
}

fn merge_unique_string_slices(v1: &[String], v2: &[String]) -> Vec<String> {
    let mut result = Vec::from(v1);
    let mut strings: HashSet<String> = v1.iter().cloned().collect();
    for string in v2 {
        if strings.insert(string.clone()) {
            result.push(string.clone());
        }
    }
    result.sort();
    result
}

// TODO: should error if strings are not the same.
fn merge_optional_unique_string_vectors(
    v1: &Option<Vec<String>>,
    v2: &Option<Vec<String>>,
) -> Option<Vec<String>> {
    match (v1, v2) {
        (Some(v1), Some(v2)) => Some(merge_unique_string_slices(v1, v2)),
        (Some(v1), None) => Some(v1.clone()),
        (None, Some(v2)) => Some(v2.clone()),
        (None, None) => None,
    }
}

fn merge_optional_producer_origins(
    p1: &Option<crate::ProducerOrigins>,
    p2: &Option<crate::ProducerOrigins>,
) -> Option<crate::ProducerOrigins> {
    match (&p1, &p2) {
        (Some(r1), Some(r2)) => Some(merge_producer_origins(r1, r2)),
        (Some(_), None) => p1.clone(),
        (None, Some(_)) => p2.clone(),
        (None, None) => None,
    }
}

fn merge_producer_origins(
    p1: &crate::ProducerOrigins,
    p2: &crate::ProducerOrigins,
) -> crate::ProducerOrigins {
    match (&p1.regions, &p2.regions) {
        (Some(r1), Some(r2)) => crate::ProducerOrigins {
            regions: Some(merge_region_lists(r1, r2)),
        },
        (Some(_), None) => p1.clone(),
        (None, Some(_)) => p2.clone(),
        (None, None) => crate::ProducerOrigins { regions: None },
    }
}

fn merge_region_lists(p1: &crate::RegionList, p2: &crate::RegionList) -> crate::RegionList {
    let mut regions = HashSet::new();
    regions.extend(p1.0.iter().cloned());
    regions.extend(p2.0.iter().cloned());
    let mut regions: Vec<_> = regions.into_iter().collect();
    regions.sort();
    crate::RegionList(regions)
}

impl crate::ProducerIds {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            vat: merge_optional_unique_string_vectors(&self.vat, &other.vat),
            wiki: merge_optional_unique_string_vectors(&self.wiki, &other.wiki),
            domains: merge_optional_unique_string_vectors(&self.domains, &other.domains),
        }
    }
}

impl crate::Report {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            title: merge_optional_strings(&self.title, &other.title),
            url: merge_optional_strings(&self.url, &other.url),
        }
    }
}

// TODO: should error if reports are not the same.
fn merge_optional_reports(
    r1: &Option<crate::Reports>,
    r2: &Option<crate::Reports>,
) -> Option<crate::Reports> {
    r1.as_ref().or(r2.as_ref()).cloned()
}

// TODO: should error if reviews are not the same.
fn merge_optional_reviews(
    r1: &Option<crate::Review>,
    r2: &Option<crate::Review>,
) -> Option<crate::Review> {
    r1.as_ref().or(r2.as_ref()).cloned()
}

impl crate::CatalogProducer {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            id: self.id.clone(),
            ids: self.ids.merge(&other.ids),
            names: merge_unique_string_slices(&self.names, &other.names),
            description: merge_optional_strings(&self.description, &other.description),
            images: merge_unique_string_slices(&self.images, &other.images),
            websites: merge_unique_string_slices(&self.websites, &other.websites),
            origins: merge_optional_producer_origins(&self.origins, &other.origins),
        }
    }
}

impl crate::ReviewProducer {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            id: self.id.clone(),
            ids: self.ids.merge(&other.ids),
            names: merge_unique_string_slices(&self.names, &other.names),
            description: merge_optional_strings(&self.description, &other.description),
            images: merge_unique_string_slices(&self.images, &other.images),
            websites: merge_unique_string_slices(&self.websites, &other.websites),
            origins: merge_optional_producer_origins(&self.origins, &other.origins),
            reports: merge_optional_reports(&self.reports, &other.reports),
            review: merge_optional_reviews(&self.review, &other.review),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_optional_strings() {
        let a = Some("a".to_string());
        let b = Some("b".to_string());
        assert_eq!(merge_optional_strings(&None, &None), None);
        assert_eq!(merge_optional_strings(&a, &None), a);
        assert_eq!(merge_optional_strings(&None, &b), b);
        assert_eq!(merge_optional_strings(&a, &b), a);
    }

    #[test]
    fn test_merge_unique_string_slices() {
        let a = "a".to_string();
        let b = "b".to_string();
        let c = "c".to_string();
        assert_eq!(
            merge_unique_string_slices(&[c.clone(), b.clone()], &[b.clone(), a.clone()]),
            vec![a, b, c]
        );
    }

    #[test]
    fn test_merge_optional_unique_string_vectors() {
        let a = "a".to_string();
        let b = "b".to_string();
        let c = "c".to_string();
        assert_eq!(merge_optional_strings(&None, &None), None);
        assert_eq!(
            merge_optional_unique_string_vectors(&Some(vec![a.clone(), b.clone()]), &None),
            Some(vec![a.clone(), b.clone()])
        );
        assert_eq!(
            merge_optional_unique_string_vectors(&None, &Some(vec![a.clone(), b.clone()])),
            Some(vec![a.clone(), b.clone()])
        );
        assert_eq!(
            merge_optional_unique_string_vectors(
                &Some(vec![c.clone(), b.clone()]),
                &Some(vec![b.clone(), a.clone()])
            ),
            Some(vec![a, b, c])
        );
    }
}
