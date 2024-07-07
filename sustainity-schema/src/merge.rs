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
            url: merge_optional_strings(&self.url, &other.url),
        }
    }
}

// TODO: should error if reports are not the same.
fn merge_optional_reports(
    r1: &Option<crate::Report>,
    r2: &Option<crate::Report>,
) -> Option<crate::Report> {
    r1.as_ref().or(r2.as_ref()).cloned()
}

impl crate::Review {
    pub fn try_merge(&self, other: &Self) -> Result<Self, ()> {
        if self.eq(other) {
            Ok(self.clone())
        } else {
            Err(())
        }
    }
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
        }
    }
}

impl crate::ReviewProducer {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            id: self.id.clone(),
            ids: self.ids.merge(&other.ids),
            report: merge_optional_reports(&self.report, &other.report),
            review: merge_optional_reviews(&self.review, &other.review),
            names: merge_unique_string_slices(&self.names, &other.names),
            description: merge_optional_strings(&self.description, &other.description),
            images: merge_unique_string_slices(&self.images, &other.images),
            websites: merge_unique_string_slices(&self.websites, &other.websites),
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
