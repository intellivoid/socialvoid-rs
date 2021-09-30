mod entities;
mod utils;

use entities::RawRequest;
use entities::RawResponse;
pub use entities::RpcError;

use utils::generate_id;

pub struct Client {
    client: reqwest::Client,
    host_url: String,
}

///Creates a new client
pub fn new(host: &str) -> Client {
    let client = reqwest::Client::new();
    Client {
        client,
        host_url: host.to_string(),
    }
}

impl Client {
    ///Send a request
    pub async fn send_request<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, RpcError> {
        let request = RawRequest::new(Some(generate_id()), method.to_string(), Some(params));

        println!(
            "Request: {}",
            serde_json::to_string_pretty(&request).unwrap()
        );

        //TODO: maybe check the response better as well??
        let resp = self
            .client
            .post(&self.host_url)
            .json(&request)
            .send()
            .await?
            .json::<RawResponse<T>>()
            .await?;

        resp.result()
    }

    /// Send a notification
    pub async fn send_notification(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<(), RpcError> {
        let request = RawRequest::new(None, method.to_string(), Some(params));

        println!(
            "Request: {}",
            serde_json::to_string_pretty(&request).unwrap()
        );

        self.client
            .post(&self.host_url)
            .json(&request)
            .send()
            .await?;

        Ok(())
    }
}

impl<T> RawResponse<T> {
    fn result(self) -> Result<T, RpcError> {
        if let Some(res) = self.result {
            return Ok(res);
        }
        if let Some(err) = self.error {
            return Err(err);
        }
        Err(RpcError {
            code: -1,
            message: Some(format!(
                "Neither result nor error was found. ID = {:?}",
                self.id
            )),
            data: None,
        })
    }
}

impl RpcError {
    /// Get error code for the RPC Error
    pub fn code(&self) -> i32 {
        self.code
    }

    //Returns the message in the error if it exists or returns `"none"`
    pub fn message(&self) -> &str {
        self.message.as_ref().map_or("none", AsRef::as_ref)
    }
}

impl std::convert::From<reqwest::Error> for RpcError {
    /// Convert a Reqwest error to an RPC Error with code -1
    fn from(error: reqwest::Error) -> Self {
        RpcError {
            code: -1,
            message: Some(format!("Reqwest error: {}", error)),
            data: None,
        }
    }
}
