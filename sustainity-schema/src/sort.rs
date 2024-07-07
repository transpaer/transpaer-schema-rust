impl crate::RegionList {
    pub fn sort(&mut self) {
        self.0.sort();
    }
}

impl crate::Regions {
    pub fn sort(&mut self) {
        match self {
            Self::Variant(_variant) => {}
            Self::List(list) => list.sort(),
        }
    }
}

impl crate::ProductAvailability {
    pub fn sort(&mut self) {
        self.regions.sort();
    }
}

impl crate::ProductCategorisation {
    pub fn sort(&mut self) {
        // TODO: self.categories.sort();
    }
}

impl crate::ProductIds {
    pub fn sort(&mut self) {
        if let Some(ean) = &mut self.ean {
            ean.sort();
        }
        if let Some(gtin) = &mut self.gtin {
            gtin.sort();
        }
        if let Some(wiki) = &mut self.wiki {
            wiki.sort();
        }
    }
}

impl crate::ProducerIds {
    pub fn sort(&mut self) {
        if let Some(wiki) = &mut self.wiki {
            wiki.sort();
        }
        if let Some(vat) = &mut self.vat {
            vat.sort();
        }
    }
}

impl crate::ProductOrigins {
    pub fn sort(&mut self) {
        self.producer_ids.sort()
    }
}

impl crate::RelatedProducts {
    pub fn sort(&mut self) {
        if let Some(followed_by) = &mut self.followed_by {
            followed_by.sort();
        }
        if let Some(preceded_by) = &mut self.preceded_by {
            preceded_by.sort();
        }
    }
}

impl crate::CatalogProduct {
    pub fn sort(&mut self) {
        if let Some(availability) = &mut self.availability {
            availability.sort();
        }
        if let Some(categorisation) = &mut self.categorisation {
            categorisation.sort();
        }
        self.ids.sort();
        self.images.sort();
        self.names.sort();
        if let Some(origins) = &mut self.origins {
            origins.sort();
        }
        if let Some(related) = &mut self.related {
            related.sort();
        }
    }
}

impl crate::CatalogProducer {
    pub fn sort(&mut self) {
        self.ids.sort();
        self.images.sort();
        self.names.sort();
        self.websites.sort();
    }
}

impl crate::ProducerProduct {
    pub fn sort(&mut self) {
        if let Some(availability) = &mut self.availability {
            availability.sort();
        }
        self.categorisation.sort();
        self.ids.sort();
        self.images.sort();
        self.names.sort();
        if let Some(origins) = &mut self.origins {
            origins.sort();
        }
        if let Some(related) = &mut self.related {
            related.sort();
        }
    }
}

impl crate::ProducerReviewer {
    pub fn sort(&mut self) {
        self.names.sort();
    }
}

impl crate::ReviewProduct {
    pub fn sort(&mut self) {
        if let Some(availability) = &mut self.availability {
            availability.sort();
        }
        if let Some(categorisation) = &mut self.categorisation {
            categorisation.sort();
        }
        self.ids.sort();
        self.images.sort();
        self.names.sort();
        if let Some(origins) = &mut self.origins {
            origins.sort();
        }
        if let Some(related) = &mut self.related {
            related.sort();
        }
    }
}

impl crate::ReviewProducer {
    pub fn sort(&mut self) {
        self.ids.sort();
        self.images.sort();
        self.names.sort();
        self.websites.sort();
    }
}

impl crate::CatalogerRoot {
    pub fn sort(&mut self) {
        for producer in &mut self.producers {
            producer.sort();
        }
        for product in &mut self.products {
            product.sort();
        }
    }
}

impl crate::ProducerRoot {
    pub fn sort(&mut self) {
        for product in &mut self.products {
            product.sort();
        }
        for reviewer in &mut self.reviewers {
            reviewer.sort();
        }
    }
}

impl crate::ReviewerRoot {
    pub fn sort(&mut self) {
        for producer in &mut self.producers {
            producer.sort();
        }
        for product in &mut self.products {
            product.sort();
        }
    }
}

impl crate::Root {
    pub fn sort(&mut self) {
        match self {
            Self::CatalogerRoot(root) => root.sort(),
            Self::ProducerRoot(root) => root.sort(),
            Self::ReviewerRoot(root) => root.sort(),
        }
    }
}
