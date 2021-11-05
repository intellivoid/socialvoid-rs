use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::{Duration, UNIX_EPOCH};

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
    width: u32,
    height: u32,
    document: Document,
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
    first_name: String,
    last_name: Option<String>,
    name: String,
    biography: Option<String>,
    location: Option<String>,
    url: Option<String>,
    followers_count: u32,
    following_count: u32,
    display_picture_sizes: Vec<DisplayPictureSize>,
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

impl std::fmt::Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Post Type: {}
ID: {}
Author: {}
Source: {}
----
{}
----
{} attachment(s)
Posted on: {}
Likes: {}, Reposts: {}, Quotes: {}, Replies: {},
Flags: ",
            match self.post_type {
                PostType::Reply => format!(
                    "Reply to <{}>",
                    match &self.reply_to_post {
                        Some(reply_to_post) => reply_to_post.id.clone(),
                        None => String::new(),
                    }
                ),
                PostType::Quote => format!(
                    "Quoted post <{}>",
                    match &self.quoted_post.as_ref() {
                        Some(quoted_post) => quoted_post.id.clone(),
                        None => String::new(),
                    }
                ),
                PostType::Repost => format!(
                    "Reposted post <{}>",
                    match &self.reposted_post.as_ref() {
                        Some(reposted_post) => reposted_post.id.clone(),
                        None => String::new(),
                    }
                ),
                _ => format!("{:?}", self.post_type),
            },
            self.id,
            self.peer
                .as_ref()
                .map(|x| format!("{}", x.username))
                .unwrap_or("<unavailable>".to_string()),
            self.source
                .as_ref()
                .unwrap_or(&"<not applicable>".to_owned()),
            self.text.as_ref().unwrap_or(&"<no text>".to_string()),
            self.attachments.len(), //TODO: maybe show the document IDs
            {
                let d = UNIX_EPOCH + Duration::from_secs(self.posted_timestamp);
                // Create DateTime from SystemTime
                let datetime = DateTime::<Utc>::from(d);
                // Formats the combined date and time with the specified format string.
                datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string()
            },
            self.like_count
                .map(|x| x.to_string())
                .unwrap_or("<not applicable>".to_string()),
            self.repost_count
                .map(|x| x.to_string())
                .unwrap_or("<not applicable>".to_string()),
            self.quote_count
                .map(|x| x.to_string())
                .unwrap_or("<not applicable>".to_string()),
            self.reply_count
                .map(|x| x.to_string())
                .unwrap_or("<not applicable>".to_string()),
        )
    }
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "First Name: {}
{}
Name: {}
{}
{}
{}
Followers: {}
Following: {}
Display Picture: {}",
            self.first_name,
            self.last_name
                .as_ref()
                .map(|x| format!("Last Name: {}", x))
                .unwrap_or_else(|| String::from("[No last name set]")),
            self.name,
            self.biography
                .as_ref()
                .map(|x| format!("Biography: {}", x))
                .unwrap_or_else(|| String::from("[No biography set]")),
            self.location
                .as_ref()
                .map(|x| format!("Location: {}", x))
                .unwrap_or_else(|| String::from("[No location set]")),
            self.url
                .as_ref()
                .map(|x| format!("URL: {}", x))
                .unwrap_or_else(|| String::from("[No URL set]")),
            self.followers_count,
            self.following_count,
            if self.display_picture_sizes.is_empty() {
                String::from("not set")
            } else {
                let count = self.display_picture_sizes.len();
                let name = &self.display_picture_sizes[0].document.file_name;
                format!("'{}' ({} sizes available)", name, count)
            }
        )
    }
}
