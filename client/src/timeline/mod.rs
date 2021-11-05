use crate::SVSessionMethods;
use serde_json::json;
use socialvoid_rawclient::Error;
use socialvoid_types::Peer;
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
    pub async fn retrieve_feed(&self, page: Option<u32>) -> Result<Vec<Post>, Error> {
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

    /// Get post from the timeline using it's ID
    pub async fn get_post(&self, post: String) -> Result<Post, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "timeline.get_post",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "post":post,
                }),
            )
            .await
    }

    /// Get likes of a post
    pub async fn get_likes(&self, post: String, page: Option<u32>) -> Result<Vec<Peer>, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "timeline.get_likes",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "post":post,
                    "page":page,
                }),
            )
            .await
    }

    /// Get replies of a post
    pub async fn get_replies(&self, post: String, page: Option<u32>) -> Result<Vec<Post>, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "timeline.get_replies",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "post":post,
                    "page":page,
                }),
            )
            .await
    }

    /// Get quotes of a post
    pub async fn get_quotes(&self, post: String, page: Option<u32>) -> Result<Vec<Post>, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "timeline.get_quotes",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "post":post,
                    "page":page,
                }),
            )
            .await
    }

    /// Like a post.
    pub async fn like(&self, post: String) -> Result<bool, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "timeline.like",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "post":post,
                }),
            )
            .await
    }

    /// Unlike a post.
    pub async fn unlike(&self, post: String) -> Result<bool, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "timeline.unlike",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "post":post,
                }),
            )
            .await
    }

    /// Compose a reply to a post.  
    /// post: ID of the post to reply to.  
    /// text: The text contents of the post to compose  
    /// attachments: Vector of document IDs to send as attachment
    pub async fn reply(
        &self,
        post: String,
        text: String,
        attachments: Vec<String>,
    ) -> Result<bool, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "timeline.reply",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "post":post,
                    "text":text,
                    "attachments": attachments,
                }),
            )
            .await
    }

    /// Compose a new post by quoting an existing post.  
    /// post: ID of the post to quote.  
    /// text: The text contents of the post to compose  
    /// attachments: Vector of document IDs to send as attachment
    pub async fn quote(
        &self,
        post: String,
        text: String,
        attachments: Vec<String>,
    ) -> Result<bool, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "timeline.quote",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "post":post,
                    "text":text,
                    "attachments": attachments,
                }),
            )
            .await
    }

    /// Repost a post.
    pub async fn repost(&self, post: String) -> Result<bool, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "timeline.repost",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "post":post,
                }),
            )
            .await
    }

    /// Get reposted peers
    pub async fn get_reposted_peers(&self, post: String, page: Option<u32>) -> Result<bool, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "timeline.get_reposted_peers",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "post":post,
                    "page": page,
                }),
            )
            .await
    }
}
