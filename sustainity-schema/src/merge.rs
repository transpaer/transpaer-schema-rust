use std::collections::HashSet;

// XXX test
fn merge_optional_strings(s1: &Option<String>, s2: &Option<String>) -> Option<String> {
    s1.as_ref().or(s2.as_ref()).cloned()
}

// XXX test
fn merge_unique_string_slices(v1: &[String], v2: &[String]) -> Vec<String> {
    let mut result = Vec::from(v1);
    let mut strings: HashSet<String> = v1.iter().cloned().collect();
    for string in v2 {
        if strings.insert(string.clone()) {
            result.push(string.clone());
        }
    }
    result
}

// XXX test
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

// XXX test
impl crate::ProducerIds {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            vat: merge_optional_unique_string_vectors(&self.vat, &other.vat),
            wiki: merge_optional_unique_string_vectors(&self.wiki, &other.wiki),
        }
    }
}

// XXX test
impl crate::Report {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            url: merge_optional_strings(&self.url, &other.url),
        }
    }
}

// XXX test
fn merge_optional_reports(
    r1: &Option<crate::Report>,
    r2: &Option<crate::Report>,
) -> Option<crate::Report> {
    // XXX TODO
    r1.as_ref().or(r2.as_ref()).cloned()
}

// XXX test
impl crate::Review {
    pub fn merge(&self, other: &Self) -> Self {
        if self.eq(other) {
            self.clone()
        } else {
            // XXX TODO
            panic!("XXX 4938");
        }
    }
}

// XXX test
fn merge_optional_reviews(
    r1: &Option<crate::Review>,
    r2: &Option<crate::Review>,
) -> Option<crate::Review> {
    // XXX TODO
    r1.as_ref().or(r2.as_ref()).cloned()
}

// XXX test
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

// XXX test
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
