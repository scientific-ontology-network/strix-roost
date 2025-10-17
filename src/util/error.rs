use horned_owl::error::HornedError;
use std::fmt::{Debug, Display, Formatter};
use std::io::Error;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub enum StrixError {
    InternalStrixError { message: String },
    HornedError { message: String },
    Error { message: String },
}

impl Display for StrixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Strix encountered an error: {:?}",
            match self {
                StrixError::InternalStrixError { message } => message,
                StrixError::HornedError { message } => message,
                StrixError::Error { message } => message,
            }
        )
    }
}

impl From<Error> for StrixError {
    fn from(value: Error) -> Self {
        StrixError::Error {
            message: value.to_string(),
        }
    }
}

impl From<HornedError> for StrixError {
    fn from(value: HornedError) -> Self {
        StrixError::HornedError {
            message: value.to_string(),
        }
    }
}

impl Serialize for StrixError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            StrixError::InternalStrixError { message } => serializer.serialize_str(message),
            StrixError::HornedError { message } => serializer.serialize_str(message),
            StrixError::Error { message } => serializer.serialize_str(message),
        }
    }
}
