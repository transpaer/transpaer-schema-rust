use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum CatalogEntry {
    #[serde(rename = "producer")]
    Producer(models::CatalogProducer),

    #[serde(rename = "product")]
    Product(models::CatalogProduct),
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ProducerEntry {
    #[serde(rename = "product")]
    Product(models::ProducerProduct),

    #[serde(rename = "reviewer")]
    Reviewer(models::ProducerReviewer),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ReviewEntry {
    #[serde(rename = "producer")]
    Producer(models::ReviewProducer),

    #[serde(rename = "product")]
    Product(models::ReviewProduct),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Data {
    Cataloger(models::CatalogerData),
    Producer(models::ProducerData),
    Reviewer(models::ReviewerData),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Substrate {
    pub meta: models::Meta,
    pub data: Data,
}
