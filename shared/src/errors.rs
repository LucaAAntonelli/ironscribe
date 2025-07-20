use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MetadataError {
    KeyNotFoundError(&'static str),
    EmptyValueError(&'static str),
    InvalidLengthError(&'static str),
}

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetadataError::KeyNotFoundError(key) => write!(f, "Key `{key}` not found in map!"),
            MetadataError::EmptyValueError(key) => write!(f, "Key `{key}` has empty value!"),
            MetadataError::InvalidLengthError(key) => write!(f, "Key `{key}` has length != 1!"),
        }
    }
}

impl Error for MetadataError {}
