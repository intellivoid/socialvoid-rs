use jsonrpc2_client::RpcError;
use std::convert::From;

//TODO: IMPLEMENT THIS

type ErrorCode = i32;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    code: ErrorCode,
    description: Option<String>,
}

#[derive(Debug)]
pub enum ErrorKind {
    ValidationError(ValidationError),
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
        let description = error.message();
        Self {
            code,
            kind,
            description,
        }
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(_error: serde_json::Error) -> Self {
        Self {}
    }
}
