use serde::{Deserialize, Serialize};

use crate::utils::generate_id;

type RawBatchRequest = Vec<RawRequest>;
pub type BatchResponse = Vec<serde_json::Value>;

/// Used to build a batch request. This object is created using the `batch_request` function of the crate
pub struct BatchRequestBuilder {
    requests: RawBatchRequest,
}

impl BatchRequestBuilder {
    pub fn new() -> BatchRequestBuilder {
        BatchRequestBuilder { requests: vec![] }
    }

    /// Add a request to the batch request
    pub fn add_request(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> &mut BatchRequestBuilder {
        self.requests.push(RawRequest::new(
            Some(generate_id()),
            method.to_string(),
            Some(params),
        ));
        self
    }

    // Add a new notification to the batch request
    pub fn add_notification(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> &mut BatchRequestBuilder {
        self.requests
            .push(RawRequest::new(None, method.to_string(), Some(params)));
        self
    }

    /// Send the batch request
    /// TODO: mmm.. maybe find a way to add the RawResponse object here somehow???
    pub fn send() -> Result<BatchResponse, Vec<RpcError>> {
        unimplemented!()
    }
}

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
