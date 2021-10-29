use serde_json::json;
use socialvoid_rawclient::Error;
use socialvoid_types::Post;
use socialvoid_types::SessionIdentification;

/// TODO: write tests for this

/// Retrieve the posts from the users timeline
pub async fn retrieve_feed(
    client: &socialvoid_rawclient::Client,
    session_identification: SessionIdentification,
    page: Option<usize>,
) -> Result<Vec<Post>, Error> {
    client
        .send_request(
            "timeline.retrieve_feed",
            json!({
                "session_identification": serde_json::to_value(session_identification)?,
                "page": page,
            }),
        )
        .await
}

/// Compose a new post to push to the timeline
pub async fn compose(
    client: &socialvoid_rawclient::Client,
    session_identification: SessionIdentification,
    text: String,
    attachments: Vec<String>,
) -> Result<Post, Error> {
    client
        .send_request(
            "timeline.compose",
            json!({
                "session_identification": serde_json::to_value(session_identification)?,
                "text":text,
                "attachments":attachments,
            }),
        )
        .await
}

/// Delete a post from the timeline using it's ID
pub async fn delete(
    client: &socialvoid_rawclient::Client,
    session_identification: SessionIdentification,
    post: String,
) -> Result<bool, Error> {
    client
        .send_request(
            "timeline.delete",
            json!({
                "session_identification": serde_json::to_value(session_identification)?,
                "post":post,
            }),
        )
        .await
}
