pub mod errors;
pub mod types;

use jsonrpc2_client::RpcError;
use std::convert::From;

use types::Error;
use types::ErrorKind;

impl From<RpcError> for Error {
    fn from(error: RpcError) -> Self {
        let code = error.code();
        let kind = ErrorKind::from(error.code());
        let description = error.message().to_string();
        Self {
            code,
            kind,
            description,
        }
    }
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
