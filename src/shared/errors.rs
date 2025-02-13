use reqwest::{header::InvalidHeaderValue, Error as ReqwestError};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, ApiError>;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    ReqwestError(#[from] ReqwestError),
    #[error("Invalid header value")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("Serialization/deserialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}
