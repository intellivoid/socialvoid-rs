use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RawRequest {
    pub jsonrpc: String,
    pub id: Option<String>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

impl RawRequest {
    pub fn new(id: Option<String>, method: String, params: Option<serde_json::Value>) -> Self {
        Self {
            jsonrpc: String::from("2.0"),
            id,
            method,
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawResponse<T> {
    pub jsonrpc: String,
    pub result: Option<T>,
    pub error: Option<RpcError>,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: Option<String>,
    pub data: Option<serde_json::Value>,
}
