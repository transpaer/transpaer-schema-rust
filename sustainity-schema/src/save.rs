use crate::{data, defs, errors};

use snafu::prelude::*;

impl crate::CatalogerRoot {
    pub fn save(&self, path: &std::path::Path) -> Result<(), errors::SaveError> {
        match defs::get_extension(path) {
            Some(defs::SubstrateExtension::Yaml) => {
                let contents =
                    serde_yaml::to_string(self).context(errors::save::YamlSnafu { path })?;
                std::fs::write(path, contents).context(errors::save::IoSnafu { path })?;
                Ok(())
            }
            Some(defs::SubstrateExtension::Json) => {
                let contents =
                    serde_json::to_string_pretty(self).context(errors::save::JsonSnafu { path })?;
                std::fs::write(path, contents).context(errors::save::IoSnafu { path })?;
                Ok(())
            }
            Some(defs::SubstrateExtension::JsonLines) => {
                serde_jsonlines::write_json_lines(path, [&self.meta])
                    .context(errors::save::JsonLinesSnafu { path })?;
                serde_jsonlines::append_json_lines(path, [&self.cataloger])
                    .context(errors::save::JsonLinesSnafu { path })?;
                serde_jsonlines::append_json_lines(
                    path,
                    self.products
                        .iter()
                        .map(|p| data::CatalogEntry::Product(p.clone())),
                )
                .context(errors::save::JsonLinesSnafu { path })?;
                serde_jsonlines::append_json_lines(
                    path,
                    self.producers
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
}

impl crate::ProducerRoot {
    pub fn save(&self, path: &std::path::Path) -> Result<(), errors::SaveError> {
        match defs::get_extension(path) {
            Some(defs::SubstrateExtension::Yaml) => {
                let contents =
                    serde_yaml::to_string(self).context(errors::save::YamlSnafu { path })?;
                std::fs::write(path, contents).context(errors::save::IoSnafu { path })?;
                Ok(())
            }
            Some(defs::SubstrateExtension::Json) => {
                let contents =
                    serde_json::to_string_pretty(self).context(errors::save::JsonSnafu { path })?;
                std::fs::write(path, contents).context(errors::save::IoSnafu { path })?;
                Ok(())
            }
            Some(defs::SubstrateExtension::JsonLines) => {
                serde_jsonlines::write_json_lines(path, [&self.meta])
                    .context(errors::save::JsonLinesSnafu { path })?;
                serde_jsonlines::append_json_lines(path, [&self.producer])
                    .context(errors::save::JsonLinesSnafu { path })?;
                serde_jsonlines::append_json_lines(
                    path,
                    self.products
                        .iter()
                        .map(|p| data::ProducerEntry::Product(p.clone())),
                )
                .context(errors::save::JsonLinesSnafu { path })?;
                serde_jsonlines::append_json_lines(
                    path,
                    self.reviewers
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
}

impl crate::ReviewerRoot {
    pub fn save(&self, path: &std::path::Path) -> Result<(), errors::SaveError> {
        match defs::get_extension(path) {
            Some(defs::SubstrateExtension::Yaml) => {
                let contents =
                    serde_yaml::to_string(self).context(errors::save::YamlSnafu { path })?;
                std::fs::write(path, contents).context(errors::save::IoSnafu { path })?;
                Ok(())
            }
            Some(defs::SubstrateExtension::Json) => {
                let contents =
                    serde_json::to_string_pretty(self).context(errors::save::JsonSnafu { path })?;
                std::fs::write(path, contents).context(errors::save::IoSnafu { path })?;
                Ok(())
            }
            Some(defs::SubstrateExtension::JsonLines) => {
                serde_jsonlines::write_json_lines(path, [&self.meta])
                    .context(errors::save::JsonLinesSnafu { path })?;
                serde_jsonlines::append_json_lines(path, [&self.reviewer])
                    .context(errors::save::JsonLinesSnafu { path })?;
                serde_jsonlines::append_json_lines(
                    path,
                    self.products
                        .iter()
                        .map(|p| data::ReviewEntry::Product(p.clone())),
                )
                .context(errors::save::JsonLinesSnafu { path })?;
                serde_jsonlines::append_json_lines(
                    path,
                    self.producers
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
}

impl crate::Root {
    pub fn save(&self, path: &std::path::Path) -> Result<(), errors::SaveError> {
        match self {
            crate::Root::CatalogerRoot(root) => root.save(path),
            crate::Root::ProducerRoot(root) => root.save(path),
            crate::Root::ReviewerRoot(root) => root.save(path),
        }
    }
}
