use crate::{data, defs, errors};

use serde::Serialize;
use snafu::prelude::*;

pub fn save_cataloger(
    path: &std::path::Path,
    meta: &crate::Meta,
    data: &crate::CatalogerData,
) -> Result<(), errors::SaveError> {
    match defs::get_extension(path) {
        Some(defs::SubstrateExtension::Yaml) => {
            let mut buffer = Vec::new();
            let mut serializer = serde_yaml::Serializer::new(&mut buffer);

            meta.serialize(&mut serializer)
                .context(errors::save::YamlSnafu { path })?;
            data.serialize(&mut serializer)
                .context(errors::save::YamlSnafu { path })?;

            std::fs::write(path, buffer).context(errors::save::IoSnafu { path })?;
            Ok(())
        }
        Some(defs::SubstrateExtension::JsonLines) => {
            serde_jsonlines::write_json_lines(path, [meta])
                .context(errors::save::JsonLinesSnafu { path })?;
            serde_jsonlines::append_json_lines(path, [&data.cataloger])
                .context(errors::save::JsonLinesSnafu { path })?;
            serde_jsonlines::append_json_lines(
                path,
                data.products
                    .iter()
                    .map(|p| data::CatalogEntry::Product(p.clone())),
            )
            .context(errors::save::JsonLinesSnafu { path })?;
            serde_jsonlines::append_json_lines(
                path,
                data.producers
                    .iter()
                    .map(|p| data::CatalogEntry::Producer(p.clone())),
            )
            .context(errors::save::JsonLinesSnafu { path })?;
            Ok(())
        }
        None => Err(errors::SubstrateError::UnsupportedExtension)
            .context(errors::save::SubstrateSnafu { path }),
    }
}

pub fn save_producer(
    path: &std::path::Path,
    meta: &crate::Meta,
    data: &crate::ProducerData,
) -> Result<(), errors::SaveError> {
    match defs::get_extension(path) {
        Some(defs::SubstrateExtension::Yaml) => {
            let mut buffer = Vec::new();
            let mut serializer = serde_yaml::Serializer::new(&mut buffer);

            meta.serialize(&mut serializer)
                .context(errors::save::YamlSnafu { path })?;
            data.serialize(&mut serializer)
                .context(errors::save::YamlSnafu { path })?;

            std::fs::write(path, buffer).context(errors::save::IoSnafu { path })?;
            Ok(())
        }
        Some(defs::SubstrateExtension::JsonLines) => {
            serde_jsonlines::write_json_lines(path, [&meta])
                .context(errors::save::JsonLinesSnafu { path })?;
            serde_jsonlines::append_json_lines(path, [&data.producer])
                .context(errors::save::JsonLinesSnafu { path })?;
            serde_jsonlines::append_json_lines(
                path,
                data.products
                    .iter()
                    .map(|p| data::ProducerEntry::Product(p.clone())),
            )
            .context(errors::save::JsonLinesSnafu { path })?;
            serde_jsonlines::append_json_lines(
                path,
                data.reviewers
                    .iter()
                    .map(|r| data::ProducerEntry::Reviewer(r.clone())),
            )
            .context(errors::save::JsonLinesSnafu { path })?;
            Ok(())
        }
        None => Err(errors::SubstrateError::UnsupportedExtension)
            .context(errors::save::SubstrateSnafu { path }),
    }
}

pub fn save_reviewer(
    path: &std::path::Path,
    meta: &crate::Meta,
    data: &crate::ReviewerData,
) -> Result<(), errors::SaveError> {
    match defs::get_extension(path) {
        Some(defs::SubstrateExtension::Yaml) => {
            let mut buffer = Vec::new();
            let mut serializer = serde_yaml::Serializer::new(&mut buffer);

            meta.serialize(&mut serializer)
                .context(errors::save::YamlSnafu { path })?;
            data.serialize(&mut serializer)
                .context(errors::save::YamlSnafu { path })?;

            std::fs::write(path, buffer).context(errors::save::IoSnafu { path })?;
            Ok(())
        }
        Some(defs::SubstrateExtension::JsonLines) => {
            serde_jsonlines::write_json_lines(path, [&meta])
                .context(errors::save::JsonLinesSnafu { path })?;
            serde_jsonlines::append_json_lines(path, [&data.reviewer])
                .context(errors::save::JsonLinesSnafu { path })?;
            serde_jsonlines::append_json_lines(
                path,
                data.products
                    .iter()
                    .map(|p| data::ReviewEntry::Product(p.clone())),
            )
            .context(errors::save::JsonLinesSnafu { path })?;
            serde_jsonlines::append_json_lines(
                path,
                data.producers
                    .iter()
                    .map(|p| data::ReviewEntry::Producer(p.clone())),
            )
            .context(errors::save::JsonLinesSnafu { path })?;
            Ok(())
        }
        None => Err(errors::SubstrateError::UnsupportedExtension)
            .context(errors::save::SubstrateSnafu { path }),
    }
}

impl crate::data::Substrate {
    pub fn save(&self, path: &std::path::Path) -> Result<(), errors::SaveError> {
        match &self.data {
            crate::data::Data::Cataloger(data) => save_cataloger(path, &self.meta, data),
            crate::data::Data::Producer(data) => save_producer(path, &self.meta, data),
            crate::data::Data::Reviewer(data) => save_reviewer(path, &self.meta, data),
        }
    }
}
