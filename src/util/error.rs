use std::{fmt, io, num::ParseFloatError, num::ParseIntError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HydroxylError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("nbt error: {0}")]
    Nbt(#[from] fast_nbt::error::Error),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("invalid cli arguments: {0}")]
    InvalidCli(String),
    #[error("invalid data: {0}")]
    InvalidData(String),
    #[error("parse int error: {0}")]
    ParseInt(#[from] ParseIntError),
    #[error("parse float error: {0}")]
    ParseFloat(#[from] ParseFloatError),
}

impl HydroxylError {
    pub fn invalid_data(message: impl Into<String>) -> Self {
        Self::InvalidData(message.into())
    }

    pub fn invalid_cli(message: impl Into<String>) -> Self {
        Self::InvalidCli(message.into())
    }
}

impl From<uuid::Error> for HydroxylError {
    fn from(value: uuid::Error) -> Self {
        Self::InvalidData(value.to_string())
    }
}

impl From<fmt::Error> for HydroxylError {
    fn from(value: fmt::Error) -> Self {
        Self::InvalidData(value.to_string())
    }
}
