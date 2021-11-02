use serde_json::json;
use socialvoid_rawclient::Error;
use socialvoid_types::SessionIdentification;
use std::sync::Arc;

pub struct SVAccountMethods {
    client: Arc<socialvoid_rawclient::Client>,
}

impl SVAccountMethods {
    pub fn new(client: Arc<socialvoid_rawclient::Client>) -> Self {
        Self { client }
    }

    pub async fn set_profile_picture(
        &self,
        session_identification: SessionIdentification,
        document_id: String,
    ) -> Result<bool, Error> {
        self.client
            .send_request(
                "account.set_profile_picture",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "document": document_id,
                }),
            )
            .await
    }
}
