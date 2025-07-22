use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[snafu(module(substrate))]
pub enum SubstrateError {
    #[snafu(display("Unsupported extension"))]
    UnsupportedExtension,

    #[snafu(display("No `meta` section"))]
    NoMeta,

    #[snafu(display("No `about` section"))]
    NoAbout,

    #[snafu(display("No `data` section"))]
    NoData,
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[snafu(module(save))]
pub enum SaveError {
    #[snafu(display("Failed to write schema (in {path:?}): {source}"))]
    Io {
        source: std::io::Error,
        path: std::path::PathBuf,
    },

    #[snafu(display("Failed to serialize as JSON (in {path:?}): {source}"))]
    Json {
        source: serde_json::Error,
        path: std::path::PathBuf,
    },

    #[snafu(display("Failed to serialize as JSON lines (in {path:?}): {source}"))]
    JsonLines {
        source: std::io::Error,
        path: std::path::PathBuf,
    },

    #[snafu(display("Failed to serialize as YAML (in {path:?}): {source}"))]
    Yaml {
        source: serde_yaml::Error,
        path: std::path::PathBuf,
    },

    #[snafu(display("Substrate error (in {path:?}): {source}"))]
    Substrate {
        source: SubstrateError,
        path: std::path::PathBuf,
    },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[snafu(module(read))]
pub enum ReadError {
    #[snafu(display("Failed to write schema (in {path:?}): {source}"))]
    Io {
        source: std::io::Error,
        path: std::path::PathBuf,
    },

    #[snafu(display("Failed to deserialize as JSON (in {path:?}): {source}"))]
    Json {
        source: serde_json::Error,
        path: std::path::PathBuf,
    },

    #[snafu(display("Failed to deserialize as YAML (in {path:?}): {source}"))]
    Yaml {
        path: std::path::PathBuf,
        source: serde_yaml::Error,
    },

    #[snafu(display("Substrate error (in {path:?}): {source}"))]
    Substrate {
        source: SubstrateError,
        path: std::path::PathBuf,
    },
}
