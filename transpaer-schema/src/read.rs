use std::io::BufRead;

use serde::Deserialize;
use snafu::prelude::*;

use crate::{
    data::{CatalogEntry, ProducerEntry, ReviewEntry},
    defs, errors,
    models::{
        AboutCataloger, AboutProducer, AboutReviewer, CatalogerData, Meta, ProducerData,
        ProviderVariant, ReviewerData,
    },
};

type Lines = std::io::Lines<std::io::BufReader<std::fs::File>>;

fn build_yaml_content_iter(path: &std::path::Path) -> Result<FileIterVariant, errors::ReadError> {
    let contents = std::fs::read_to_string(path).context(errors::read::IoSnafu { path })?;
    let mut deserializer = serde_yaml::Deserializer::from_str(&contents);

    let meta: Meta = if let Some(header) = deserializer.next() {
        Meta::deserialize(header).context(errors::read::YamlSnafu { path })?
    } else {
        return Err(errors::SubstrateError::NoMeta).context(errors::read::SubstrateSnafu { path });
    };

    let data = if let Some(data) = deserializer.next() {
        data
    } else {
        return Err(errors::SubstrateError::NoData).context(errors::read::SubstrateSnafu { path });
    };

    match meta.variant {
        ProviderVariant::Cataloger => {
            let data =
                CatalogerData::deserialize(data).context(errors::read::YamlSnafu { path })?;
            Ok(FileIterVariant::Catalog(CatalogIter::from_data(data)))
        }
        ProviderVariant::Producer => {
            let data = ProducerData::deserialize(data).context(errors::read::YamlSnafu { path })?;
            Ok(FileIterVariant::Producer(ProducerIter::from_data(data)))
        }
        ProviderVariant::Reviewer => {
            let data = ReviewerData::deserialize(data).context(errors::read::YamlSnafu { path })?;
            Ok(FileIterVariant::Review(ReviewIter::from_data(data)))
        }
    }
}

fn build_lines_iter(path: &std::path::Path) -> Result<FileIterVariant, errors::ReadError> {
    let file = std::fs::File::open(path).context(errors::read::IoSnafu { path })?;
    let mut lines = std::io::BufReader::new(file).lines();

    if let Some(meta_str) = lines.next() {
        let meta_str = meta_str.context(errors::read::IoSnafu { path })?;
        let meta: Meta =
            serde_json::from_str(&meta_str).context(errors::read::JsonSnafu { path })?;
        if let Some(about_str) = lines.next() {
            let about_str = about_str.context(errors::read::IoSnafu { path })?;
            Ok(match meta.variant {
                ProviderVariant::Cataloger => {
                    let _about: AboutCataloger = serde_json::from_str(&about_str)
                        .context(errors::read::JsonSnafu { path })?;
                    FileIterVariant::Catalog(CatalogIter::from_lines(lines, path.to_owned()))
                }
                ProviderVariant::Producer => {
                    let _about: AboutProducer = serde_json::from_str(&about_str)
                        .context(errors::read::JsonSnafu { path })?;
                    FileIterVariant::Producer(ProducerIter::from_lines(lines, path.to_owned()))
                }
                ProviderVariant::Reviewer => {
                    let _about: AboutReviewer = serde_json::from_str(&about_str)
                        .context(errors::read::JsonSnafu { path })?;
                    FileIterVariant::Review(ReviewIter::from_lines(lines, path.to_owned()))
                }
            })
        } else {
            Err(errors::SubstrateError::NoAbout).context(errors::read::SubstrateSnafu { path })
        }
    } else {
        Err(errors::SubstrateError::NoMeta).context(errors::read::SubstrateSnafu { path })
    }
}

struct ContentCatalogIter {
    content: Vec<CatalogEntry>,
    index: usize,
}

impl ContentCatalogIter {
    fn from_data(data: CatalogerData) -> Self {
        let mut content = Vec::with_capacity(data.products.len() + data.producers.len());
        for product in data.products {
            content.push(CatalogEntry::Product(product))
        }
        for producer in data.producers {
            content.push(CatalogEntry::Producer(producer))
        }
        Self { content, index: 0 }
    }
}

impl std::iter::Iterator for ContentCatalogIter {
    type Item = Result<CatalogEntry, errors::ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.content.get(self.index - 1).map(|e| Ok(e.clone()))
    }
}

struct LazyCatalogIter {
    path: std::path::PathBuf,
    lines: Lines,
}

impl LazyCatalogIter {
    fn from_lines(lines: Lines, path: std::path::PathBuf) -> Self {
        Self { lines, path }
    }
}

impl std::iter::Iterator for LazyCatalogIter {
    type Item = Result<CatalogEntry, errors::ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(string) = self.lines.next() {
            Some(match string {
                Ok(ok) => serde_json::from_str(&ok).context(errors::read::JsonSnafu {
                    path: self.path.clone(),
                }),
                Err(err) => Err(err).context(errors::read::IoSnafu {
                    path: self.path.clone(),
                }),
            })
        } else {
            None
        }
    }
}

enum InnerCatalogIter {
    Content(ContentCatalogIter),
    Lazy(LazyCatalogIter),
}

pub struct CatalogIter {
    inner: InnerCatalogIter,
}

impl CatalogIter {
    fn from_data(data: CatalogerData) -> Self {
        Self {
            inner: InnerCatalogIter::Content(ContentCatalogIter::from_data(data)),
        }
    }

    fn from_lines(lines: Lines, path: std::path::PathBuf) -> Self {
        Self {
            inner: InnerCatalogIter::Lazy(LazyCatalogIter::from_lines(lines, path)),
        }
    }
}

impl std::iter::Iterator for CatalogIter {
    type Item = Result<CatalogEntry, errors::ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            InnerCatalogIter::Content(iter) => iter.next(),
            InnerCatalogIter::Lazy(iter) => iter.next(),
        }
    }
}

struct ContentProducerIter {
    content: Vec<ProducerEntry>,
    index: usize,
}

impl ContentProducerIter {
    fn from_data(data: ProducerData) -> Self {
        let mut content = Vec::with_capacity(data.products.len() + data.reviewers.len());
        for product in data.products {
            content.push(ProducerEntry::Product(product))
        }
        for reviewer in data.reviewers {
            content.push(ProducerEntry::Reviewer(reviewer))
        }
        Self { content, index: 0 }
    }
}

impl std::iter::Iterator for ContentProducerIter {
    type Item = Result<ProducerEntry, errors::ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.content.get(self.index - 1).map(|e| Ok(e.clone()))
    }
}

struct LazyProducerIter {
    path: std::path::PathBuf,
    lines: Lines,
}

impl LazyProducerIter {
    fn from_lines(lines: Lines, path: std::path::PathBuf) -> Self {
        Self { lines, path }
    }
}

impl std::iter::Iterator for LazyProducerIter {
    type Item = Result<ProducerEntry, errors::ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(string) = self.lines.next() {
            Some(match string {
                Ok(ok) => serde_json::from_str(&ok).context(errors::read::JsonSnafu {
                    path: self.path.clone(),
                }),
                Err(err) => Err(err).context(errors::read::IoSnafu {
                    path: self.path.clone(),
                }),
            })
        } else {
            None
        }
    }
}

enum InnerProducerIter {
    Content(ContentProducerIter),
    Lazy(LazyProducerIter),
}

pub struct ProducerIter {
    inner: InnerProducerIter,
}

impl ProducerIter {
    fn from_data(data: ProducerData) -> Self {
        Self {
            inner: InnerProducerIter::Content(ContentProducerIter::from_data(data)),
        }
    }

    fn from_lines(lines: Lines, path: std::path::PathBuf) -> Self {
        Self {
            inner: InnerProducerIter::Lazy(LazyProducerIter::from_lines(lines, path)),
        }
    }
}

impl std::iter::Iterator for ProducerIter {
    type Item = Result<ProducerEntry, errors::ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            InnerProducerIter::Content(iter) => iter.next(),
            InnerProducerIter::Lazy(iter) => iter.next(),
        }
    }
}

struct ContentReviewIter {
    content: Vec<ReviewEntry>,
    index: usize,
}

impl ContentReviewIter {
    fn from_data(data: ReviewerData) -> Self {
        let mut content = Vec::with_capacity(data.products.len() + data.producers.len());
        for product in data.products {
            content.push(ReviewEntry::Product(product))
        }
        for producer in data.producers {
            content.push(ReviewEntry::Producer(producer))
        }
        Self { content, index: 0 }
    }
}

impl std::iter::Iterator for ContentReviewIter {
    type Item = Result<ReviewEntry, errors::ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.content.get(self.index - 1).map(|e| Ok(e.clone()))
    }
}

struct LazyReviewIter {
    path: std::path::PathBuf,
    lines: Lines,
}

impl LazyReviewIter {
    fn from_lines(lines: Lines, path: std::path::PathBuf) -> Self {
        Self { lines, path }
    }
}

impl std::iter::Iterator for LazyReviewIter {
    type Item = Result<ReviewEntry, errors::ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(string) = self.lines.next() {
            Some(match string {
                Ok(ok) => serde_json::from_str(&ok).context(errors::read::JsonSnafu {
                    path: self.path.clone(),
                }),
                Err(err) => Err(err).context(errors::read::IoSnafu {
                    path: self.path.clone(),
                }),
            })
        } else {
            None
        }
    }
}

enum InnerReviewIter {
    Content(ContentReviewIter),
    Lazy(LazyReviewIter),
}

pub struct ReviewIter {
    inner: InnerReviewIter,
}

impl ReviewIter {
    fn from_data(data: ReviewerData) -> Self {
        Self {
            inner: InnerReviewIter::Content(ContentReviewIter::from_data(data)),
        }
    }

    fn from_lines(lines: Lines, path: std::path::PathBuf) -> Self {
        Self {
            inner: InnerReviewIter::Lazy(LazyReviewIter::from_lines(lines, path)),
        }
    }
}

impl std::iter::Iterator for ReviewIter {
    type Item = Result<ReviewEntry, errors::ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            InnerReviewIter::Content(iter) => iter.next(),
            InnerReviewIter::Lazy(iter) => iter.next(),
        }
    }
}

pub enum FileIterVariant {
    Catalog(CatalogIter),
    Producer(ProducerIter),
    Review(ReviewIter),
}

pub fn iter_file(path: &std::path::Path) -> Result<FileIterVariant, errors::ReadError> {
    match defs::get_extension(path) {
        Some(defs::SubstrateExtension::Yaml) => build_yaml_content_iter(path),
        Some(defs::SubstrateExtension::JsonLines) => build_lines_iter(path),
        None => Err(errors::ReadError::Substrate {
            source: errors::SubstrateError::UnsupportedExtension,
            path: path.to_owned(),
        }),
    }
}
