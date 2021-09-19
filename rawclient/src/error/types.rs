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
    Validation(ValidationError),
    Server(ServerError),
    Rpc(RpcError),
    JsonParsing,
    Unknown,
}

impl From<ErrorCode> for ErrorKind {
    fn from(code: ErrorCode) -> Self {
        if (-32768..=-32000).contains(&code) {
            match RpcError::from_i32(code) {
                Some(kind) => Self::Rpc(kind),
                None => Self::Unknown,
            }
        } else if (8448..=8703).contains(&code) {
            match ValidationError::from_i32(code) {
                Some(kind) => Self::Validation(kind),
                None => Self::Unknown,
            }
        } else if (16384..).contains(&code) {
            match ServerError::from_i32(code) {
                Some(kind) => Self::Server(kind),
                None => Self::Unknown,
            }
        } else {
            Self::Unknown
        }
    }
}
