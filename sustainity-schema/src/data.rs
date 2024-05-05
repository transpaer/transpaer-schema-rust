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
