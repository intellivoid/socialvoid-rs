use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionIdentification {
    pub session_id: String,
    pub client_public_hash: String,
    pub challenge_answer: String,
}

/// A Peer Object that contains information about the peer
#[derive(Serialize, Deserialize, Debug)]
pub struct Peer {
    pub id: String,
    #[serde(rename = "type")]
    pub peer_type: PeerType,
    pub name: String,
    pub username: String,
    pub flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PeerType {
    USER,
    BOT,
    PROXY,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayPictureSize {
    pub width: u32,
    pub height: u32,
    pub document: Document,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub id: String,
    pub file_mime: String,
    pub file_name: String,
    pub file_size: u32,
    pub file_type: FileType,
    pub flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileType {
    DOCUMENT,
    PHOTO,
    VIDEO,
    AUDIO,
}

/// HelpDocument -> https://github.com/intellivoid/Socialvoid-Standard-Documentation/blob/master/Objects/HelpDocument.md
#[derive(Serialize, Deserialize, Debug)]
pub struct HelpDocument {
    pub id: String,
    pub text: String,
    pub entities: Vec<TextEntity>,
}

impl HelpDocument {
    pub fn get_plain_text(&self) -> String {
        self.text.clone()
    }

    pub fn get_markdown(&self) -> String {
        //TODO: implement this
        unimplemented!()
    }

    pub fn get_html(&self) -> String {
        //TODO: implement this
        unimplemented!()
    }
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
    pub network_name: String,
    pub protocol_version: String,
    pub cdn_server: String,
    pub upload_max_file_size: u32,
    pub unauthorized_session_ttl: u32,
    pub authorized_session_ttl: u32,
    pub retrieve_likes_max_limit: u32,
    pub retrieve_reposts_max_limit: u32,
    pub retrieve_replies_max_limit: u32,
    pub retrieve_quotes_max_limit: u32,
    pub retrieve_followers_max_limit: u32,
    pub retrieve_following_max_limit: u32,
    pub retrieve_feed_max_limit: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub first_name: String,
    pub last_name: Option<String>,
    pub name: String,
    pub biography: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub followers_count: u32,
    pub following_count: u32,
    pub display_picture_sizes: Vec<DisplayPictureSize>,
}

/// Relationship of a peer with another peer.
/// https://github.com/intellivoid/Socialvoid-Standard-Documentation/blob/master/Types/RelationshipTypes.md
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelationshipType {
    None,
    Following,
    FollowsYou,
    AwaitingApproval,
    MutuallyFollowing,
    Blocked,
    BlockedYou,
}

/// Post
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: String,
    #[serde(rename = "type")]
    pub post_type: PostType,
    pub peer: Option<Peer>,
    pub source: Option<String>,
    pub text: Option<String>,
    pub attachments: Vec<Document>,
    pub entities: Vec<TextEntity>,
    pub mentioned_peers: Vec<Peer>,
    pub reply_to_post: Option<Box<Post>>,
    pub quoted_post: Option<Box<Post>>,
    pub reposted_post: Option<Box<Post>>,
    pub original_thread_post: Option<Box<Post>>,
    pub like_count: Option<usize>,
    pub repost_count: Option<usize>,
    pub quote_count: Option<usize>,
    pub reply_count: Option<usize>,
    pub posted_timestamp: u64,
    pub flags: Vec<String>,
}

/// Post Type
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PostType {
    Unknown,
    Deleted,
    Post,
    Reply,
    Quote,
    Repost,
}
