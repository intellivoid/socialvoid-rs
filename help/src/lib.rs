use rawclient::Error;
use types::{HelpDocument, ServerInformation};

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
