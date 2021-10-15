pub mod errors;
pub mod types;

use jsonrpc2_client::RpcError;
use std::convert::From;

pub use self::types::Error;
pub use self::types::ErrorKind;

use crate::CdnResponse;

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
            kind: ErrorKind::JsonParsing,
            description: String::from("JSON Parsing error"), //TODO: make more descriptive if possible
        }
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self {
            code: -1, //TODO: maybe see standard error code for this?
            kind: ErrorKind::RequestError(error),
            description: String::from("Request error occurred"), //TODO: make more descriptive if possible
        }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self {
            code: -1, //TODO: maybe see standard error code for this?
            kind: ErrorKind::IO(error),
            description: String::from("IO error occurred"), //TODO: make more descriptive if possible
        }
    }
}

impl<T> std::convert::TryFrom<&CdnResponse<T>> for Error {
    type Error = Error;
    fn try_from(resp: &CdnResponse<T>) -> Result<Self, Self::Error> {
        if resp.success {
            return Ok(Self {
                code: 0,
                kind: ErrorKind::Cdn("Unknown error. Success is true".to_string()),
                description: String::from("Unknown CDN error"),
            });
        }
        Ok(Self {
            code: resp.error_code.unwrap_or(0),
            kind: ErrorKind::Cdn(resp.message.clone().unwrap_or("".to_string())),
            description: String::from("CDN error occurred"), //TODO: make more descriptive if possible
        })
    }
}
