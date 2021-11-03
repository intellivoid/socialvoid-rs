use crate::SVSessionMethods;
use serde_json::json;
use socialvoid_rawclient::Error;
use std::sync::Arc;

pub struct SVAccountMethods {
    client: Arc<socialvoid_rawclient::Client>,
    session: Arc<SVSessionMethods>,
}

impl SVAccountMethods {
    pub fn new(client: Arc<socialvoid_rawclient::Client>, session: Arc<SVSessionMethods>) -> Self {
        Self { client, session }
    }

    pub async fn set_profile_picture(&self, document_id: String) -> Result<bool, Error> {
        let session_identification = self.session.session_identification()?;
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
