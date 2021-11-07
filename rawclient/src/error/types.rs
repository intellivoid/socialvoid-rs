use super::errors::AuthenticationError;
use super::errors::ClientError;
use super::errors::NetworkError;
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
    Authentication(AuthenticationError),
    Network(NetworkError),
    Server(ServerError),
    Rpc(RpcError),
    Cdn(String),
    JsonParsing,
    RequestError(reqwest::Error),
    IO(std::io::Error),
    Client(ClientError),
    Unknown,
}

impl Error {
    pub fn new_client_error(error_type: ClientError) -> Self {
        Self {
            kind: ErrorKind::Client(error_type),
            code: -1,
            description: String::from("There was an error on the client"), // TODO: maybe have description based on error type
        }
    }
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
        } else if (8704..=8979).contains(&code) {
            match AuthenticationError::from_i32(code) {
                Some(kind) => Self::Authentication(kind),
                None => Self::Unknown,
            }
        } else if (12544..=16383).contains(&code) {
            match NetworkError::from_i32(code) {
                Some(kind) => Self::Network(kind),
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
