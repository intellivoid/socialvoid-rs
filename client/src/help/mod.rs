use serde_json::json;
use socialvoid_rawclient::Error;
use socialvoid_types::{HelpDocument, ServerInformation};
use std::sync::Arc;

pub struct SVHelpMethods {
    client: Arc<socialvoid_rawclient::Client>,
}

impl SVHelpMethods {
    pub fn new(client: Arc<socialvoid_rawclient::Client>) -> SVHelpMethods {
        SVHelpMethods { client }
    }

    pub async fn get_community_guidelines(&self) -> Result<HelpDocument, Error> {
        self.client
            .send_request("help.get_community_guidelines", json!(null))
            .await
    }

    pub async fn get_privacy_policy(&self) -> Result<HelpDocument, Error> {
        self.client
            .send_request("help.get_privacy_policy", json!(null))
            .await
    }

    pub async fn get_server_information(&self) -> Result<ServerInformation, Error> {
        self.client
            .send_request("help.get_server_information", json!(null))
            .await
    }

    pub async fn get_terms_of_service(&self) -> Result<HelpDocument, Error> {
        self.client
            .send_request("help.get_terms_of_service", json!(null))
            .await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    pub async fn save_all_documents() {
        use std::fs::File;
        let client = socialvoid_rawclient::new();
        let help = SVHelpMethods::new(Arc::new(client));
        serde_json::to_writer(
            &File::create("community_guidelines.json.test").unwrap(),
            &help.get_community_guidelines().await.unwrap(),
        )
        .unwrap();
        serde_json::to_writer(
            &File::create("privacy_policy.json.test").unwrap(),
            &help.get_privacy_policy().await.unwrap(),
        )
        .unwrap();
        serde_json::to_writer(
            &File::create("server_information.json.test").unwrap(),
            &help.get_server_information().await.unwrap(),
        )
        .unwrap();
        serde_json::to_writer(
            &File::create("terms_of_service.json.test").unwrap(),
            &help.get_terms_of_service().await.unwrap(),
        )
        .unwrap();
    }
}
