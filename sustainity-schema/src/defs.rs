/// XXX
#[derive(Debug, Clone, Copy)]
pub enum SubstrateExtension {
    Yaml,
    Json,
    JsonLines,
}

impl SubstrateExtension {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Yaml => "yaml",
            Self::Json => "json",
            Self::JsonLines => "jsonl",
        }
    }
}

pub fn get_extension(path: &std::path::Path) -> Option<SubstrateExtension> {
    match path.extension() {
        Some(e) => {
            if e == "yaml" {
                Some(SubstrateExtension::Yaml)
            } else if e == "json" {
                Some(SubstrateExtension::Json)
            } else if e == "jsonl" {
                Some(SubstrateExtension::JsonLines)
            } else {
                None
            }
        }
        None => None,
    }
}
