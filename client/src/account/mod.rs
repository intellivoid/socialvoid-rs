use serde_json::json;
use socialvoid_rawclient::Error;
use socialvoid_types::SessionIdentification;

pub async fn set_profile_picture(
    client: &socialvoid_rawclient::Client,
    session_identification: SessionIdentification,
    document_id: String,
) -> Result<bool, Error> {
    client
        .send_request(
            "account.set_profile_picture",
            json!({
                "session_identification": serde_json::to_value(session_identification),
                "document": document_id,
            }),
        )
        .await
}
