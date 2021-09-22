use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub struct Client {
    client: reqwest::Client,
    host_url: String,
}

pub fn new(host: &str) -> Client {
    let client = reqwest::Client::new();
    Client {
        client,
        host_url: host.to_string(),
    }
}

impl Client {
    pub async fn send_request<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, RpcError> {
        let request = RawRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(generate_id()),
            method: method.to_string(),
            params: Some(params),
        };

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
}

#[derive(Debug, Serialize, Deserialize)]
struct RawRequest {
    jsonrpc: String,
    id: Option<String>,
    method: String,
    params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawResponse<T> {
    jsonrpc: String,
    result: Option<T>,
    error: Option<RpcError>,
    id: String,
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

fn generate_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    code: i32,
    message: Option<String>,
    data: Option<serde_json::Value>,
}

impl RpcError {
    pub fn code(&self) -> i32 {
        self.code
    }
    pub fn message(&self) -> &str {
        self.message.as_ref().map_or("none", AsRef::as_ref)
    }
}

impl std::convert::From<reqwest::Error> for RpcError {
    fn from(error: reqwest::Error) -> Self {
        RpcError {
            code: -1,
            message: Some(format!("An error occurred: {}", error)),
            data: None,
        }
    }
}
