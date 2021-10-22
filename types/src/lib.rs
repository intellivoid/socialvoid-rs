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
pub enum RelationshipType {
    NONE,
    FOLLOWING,
    FOLLOWS_YOU,
    AWAITING_APPROVAL,
    MUTUALLY_FOLLOWING,
    BLOCKED,
    BLOCKED_YOU,
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
                .unwrap_or(String::from("[No last name set]")),
            self.name,
            self.biography
                .as_ref()
                .map(|x| format!("Biography: {}", x))
                .unwrap_or(String::from("[No biography set]")),
            self.location
                .as_ref()
                .map(|x| format!("Location: {}", x))
                .unwrap_or(String::from("[No location set]")),
            self.url
                .as_ref()
                .map(|x| format!("URL: {}", x))
                .unwrap_or(String::from("[No URL set]")),
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
