/// # Raw client for SocialVoid.
/// Makes a new client and makes JSONRPC requests. Also, useful in case we
/// want to switch the JSONRPC client crate used in the future.
mod error;

#[macro_use]
extern crate enum_primitive;
// use futures::stream::TryStreamExt;
use std::convert::TryFrom;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

// use tokio::io::AsyncReadExt;

pub use error::errors::AuthenticationError;
pub use error::errors::ClientError;
pub use error::errors::ValidationError;
pub use error::Error;
pub use error::ErrorKind;

use socialvoid_types::Document;
use socialvoid_types::SessionIdentification;

use reqwest::multipart::Part;
use reqwest::Body;
use serde::Deserialize;

const HOST: &str = "http://socialvoid.qlg1.com:5601/";
const CDN_URL: &str = "http://socialvoid.qlg1.com:5602/";

pub struct Client {
    client: jsonrpc2_client::Client,
}

pub fn new() -> Client {
    let host = get_host();
    Client {
        client: jsonrpc2_client::new(&host),
    }
}

impl Client {
    pub async fn send_request<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, Error> {
        let response = self.client.send_request::<T>(method, params).await?;
        Ok(response)
    }
}

pub struct CdnClient {
    client: reqwest::Client,
    host_url: String,
}

impl CdnClient {
    pub fn new() -> CdnClient {
        CdnClient {
            client: reqwest::Client::new(),
            host_url: get_cdn_url(),
        }
    }

    pub fn with_cdn_url(host_url: String) -> CdnClient {
        CdnClient {
            client: reqwest::Client::new(),
            host_url,
        }
    }

    pub async fn upload(
        &self,
        session_identification: SessionIdentification,
        file_path: String,
    ) -> Result<Document, Error> {
        // let mut file_bytes = vec![];
        let document = tokio::fs::File::open(&file_path).await?;
        // document.read_to_end(&mut file_bytes).await?;
        let form = reqwest::multipart::Form::new()
            .part(
                "document",
                Part::stream(file_to_body(document)).file_name(file_path),
            )
            // .part("document", Part::bytes(file_bytes).file_name(file_path))
            .text(
                "client_public_hash",
                session_identification.client_public_hash,
            )
            .text("session_id", session_identification.session_id)
            .text("challenge_answer", session_identification.challenge_answer)
            .text("action", "upload");

        let resp: CdnResponse<Document> = self
            .client
            .post(&self.host_url)
            .multipart(form)
            .send()
            .await?
            .json()
            .await?;
        resp.results()
    }

    pub async fn download(
        &self,
        session_identification: SessionIdentification,
        document_id: String,
    ) -> Result<Vec<u8>, Error> {
        let form = reqwest::multipart::Form::new()
            .text("document", document_id)
            .text(
                "client_public_hash",
                session_identification.client_public_hash, //remove the clonse
            )
            .text("session_id", session_identification.session_id)
            .text("challenge_answer", session_identification.challenge_answer)
            .text("action", "download");

        let response = self
            .client
            .post(&self.host_url)
            .multipart(form)
            .send()
            .await?;
        let content = response.text().await?;
        Ok(content.as_bytes().to_vec())
    }
}

impl Default for CdnClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize, Debug)]
pub struct CdnResponse<T> {
    success: bool,
    error_code: Option<i32>,
    message: Option<String>,
    results: Option<T>,
}

impl<T> CdnResponse<T> {
    pub fn results(self) -> Result<T, Error> {
        if !self.success {
            let err = Error::try_from(&self)?;
            return Err(err);
        }
        match self.results {
            Some(res) => Ok(res),
            None => Err(Error {
                kind: ErrorKind::Unknown,
                code: -1,
                description: "CDN Error: Success is true but no results found".to_string(),
            }),
        }
    }
}

fn get_cdn_url() -> String {
    CDN_URL.to_string()
}

fn get_host() -> String {
    HOST.to_string()
}

fn file_to_body(file: File) -> Body {
    Body::wrap_stream(FramedRead::new(file, BytesCodec::new()))
}
