use thiserror::Error;

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("Network error: {0}")]
    Request(String),

    #[error("Error reading response: {0}")]
    ReadResponse(String),

    #[error("JSON parse error: {0}")]
    ParseJson(String),

    #[error(transparent)]
    Fs(#[from] crate::core::errors::fs::FsError),

    #[error("No Fabric loader found for this version")]
    NoFabricLoader,
}

impl DownloadError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Request(_) => "DL_CANT_FETCH",
            Self::ReadResponse(_) => "DL_CANT_READ",
            Self::ParseJson(_) => "DL_JSON_PARSE",
            Self::Fs(e) => e.code(),
            Self::NoFabricLoader => "DL_NO_FABRIC",
        }
    }

    pub fn params(&self) -> Vec<(&'static str, String)> {
        match self {
            Self::Request(s) | Self::ReadResponse(s) | Self::ParseJson(s) => {
                vec![("error", s.clone())]
            }
            Self::Fs(e) => e.params(),
            _ => vec![],
        }
    }
}
