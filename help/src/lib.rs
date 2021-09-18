use rawclient::Error;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate serde_json;

pub async fn get_community_guidelines(client: &rawclient::Client) -> Result<HelpDocument, Error> {
    client
        .send_request("help.get_community_guidelines", json!(null))
        .await
}

pub async fn get_privacy_policy(client: &rawclient::Client) -> Result<HelpDocument, Error> {
    client
        .send_request("help.get_privacy_policy", json!(null))
        .await
}

pub async fn get_server_information(
    client: &rawclient::Client,
) -> Result<ServerInformation, Error> {
    client
        .send_request("help.get_server_information", json!(null))
        .await
}

pub async fn get_terms_of_service(client: &rawclient::Client) -> Result<HelpDocument, Error> {
    client
        .send_request("help.get_terms_of_service", json!(null))
        .await
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelpDocument {
    id: String,
    text: String,
    entities: Vec<TextEntity>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextEntity {
    #[serde(rename = "type")]
    entity_type: TextEntityType,
    offset: u32,
    length: u32,
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TextEntityType {
    BOLD,
    ITALIC,
    CODE,
    STRIKE,
    UNDERLINE,
    URL,
    MENTION,
    HASHTAG,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInformation {
    network_name: String,
    protocol_version: String,
    cdn_server: String,
    upload_max_file_size: u32,
    unauthorized_session_ttl: u32,
    authorized_session_ttl: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    pub async fn save_all_documents() {
        use std::fs::File;
        let client = rawclient::new();
        serde_json::to_writer(
            &File::create("community_guidelines.json.test").unwrap(),
            &get_community_guidelines(&client).await.unwrap(),
        )
        .unwrap();
        serde_json::to_writer(
            &File::create("privacy_policy.json.test").unwrap(),
            &get_privacy_policy(&client).await.unwrap(),
        )
        .unwrap();
        serde_json::to_writer(
            &File::create("server_information.json.test").unwrap(),
            &get_server_information(&client).await.unwrap(),
        )
        .unwrap();
        serde_json::to_writer(
            &File::create("terms_of_service.json.test").unwrap(),
            &get_terms_of_service(&client).await.unwrap(),
        )
        .unwrap();
    }
}
