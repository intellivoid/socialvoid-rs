use crate::SVSessionMethods;
use serde_json::json;
use socialvoid_rawclient::Error;
use socialvoid_types::Post;

use std::sync::Arc;

/// TODO: write tests for this

pub struct SVTimelineMethods {
    client: Arc<socialvoid_rawclient::Client>,
    session: Arc<SVSessionMethods>,
}

impl SVTimelineMethods {
    pub fn new(client: Arc<socialvoid_rawclient::Client>, session: Arc<SVSessionMethods>) -> Self {
        Self { client, session }
    }
    /// Retrieve the posts from the users timeline
    pub async fn retrieve_feed(&self, page: Option<usize>) -> Result<Vec<Post>, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
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
    pub async fn compose(&self, text: &str, attachments: Vec<String>) -> Result<Post, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
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
    pub async fn delete(&self, post: String) -> Result<bool, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "timeline.delete",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "post":post,
                }),
            )
            .await
    }
}
