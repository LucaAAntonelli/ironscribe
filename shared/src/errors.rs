use std::error::Error;
use std::fmt;
use tonic::{Code, Status};

#[derive(Debug, PartialEq)]
pub enum MetadataError {
    KeyNotFoundError(&'static str),
    EmptyValueError(&'static str),
    InvalidLengthError(&'static str),
    InvalidFormatError(&'static str, String),
}

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetadataError::KeyNotFoundError(key) => write!(f, "Key `{key}` not found in map!"),
            MetadataError::EmptyValueError(key) => write!(f, "Key `{key}` has empty value!"),
            MetadataError::InvalidLengthError(key) => write!(f, "Key `{key}` has length != 1!"),
            MetadataError::InvalidFormatError(key, value) => {
                write!(f, "Value `{value}` of key `{key}` could not be parsed!")
            }
        }
    }
}

impl Error for MetadataError {}

// Implementing this trait allows usage of `?` operator on Result<(), MetadataError> in functions
// defined in protobuf
impl From<MetadataError> for Status {
    fn from(err: MetadataError) -> Self {
        let code = match &err {
            // Match custom error variants to fitting tonic Status variants, then return status
            // with message defined in Display implementation
            MetadataError::InvalidFormatError(_, _) => Code::InvalidArgument,
            MetadataError::KeyNotFoundError(_) => Code::NotFound,
            MetadataError::EmptyValueError(_) => Code::NotFound,
            MetadataError::InvalidLengthError(_) => Code::InvalidArgument,
        };
        Status::new(code, err.to_string())
    }
}
