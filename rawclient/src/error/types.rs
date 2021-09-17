use super::errors::RpcError;
use super::errors::ServerError;
use super::errors::ValidationError;

use crate::enum_primitive::FromPrimitive;

pub type ErrorCode = i32;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub code: ErrorCode,
    pub description: String,
}

#[derive(Debug)]
pub enum ErrorKind {
    ValidationError(ValidationError),
    ServerError(ServerError),
    RpcError(RpcError),
    JsonParsingError,
    UnknownError,
}

impl From<ErrorCode> for ErrorKind {
    fn from(code: ErrorCode) -> Self {
        if code >= -32768 && code <= -32000 {
            match RpcError::from_i32(code) {
                Some(kind) => Self::RpcError(kind),
                None => Self::UnknownError,
            }
        } else if code >= 8448 && code <= 8703 {
            match ValidationError::from_i32(code) {
                Some(kind) => Self::ValidationError(kind),
                None => Self::UnknownError,
            }
        } else if code >= 16384 {
            match ServerError::from_i32(code) {
                Some(kind) => Self::ServerError(kind),
                None => Self::UnknownError,
            }
        } else {
            Self::UnknownError
        }
    }
}
