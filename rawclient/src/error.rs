use jsonrpc2_client::RpcError;
use std::convert::From;

//TODO: IMPLEMENT THIS

type ErrorCode = i32;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    code: ErrorCode,
    description: String,
}

#[derive(Debug)]
pub enum ErrorKind {
    ValidationError(ValidationError),
    JsonParsingError,
}

#[derive(Debug)]
enum ValidationError {
    InvalidUsername,
    InvalidPassword,
}

impl From<RpcError> for Error {
    fn from(error: RpcError) -> Self {
        let code = error.code();
        let kind = get_error_kind(error.code());
        let description = error.message().to_string();
        Self {
            code,
            kind,
            description,
        }
    }
}

pub fn get_error_kind(code: ErrorCode) -> ErrorKind {
    //FAKE.. TODO: this.
    ErrorKind::ValidationError(ValidationError::InvalidUsername)
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(_error: serde_json::Error) -> Self {
        Self {
            code: -1, //TODO: maybe see standard error code for this?
            kind: ErrorKind::JsonParsingError,
            description: String::from("JSON Parsing error"), //TODO: make more descriptive if possible
        }
    }
}
