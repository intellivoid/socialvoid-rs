use serde::{Deserialize, Serialize};

/// A Peer Object that contains information about the peer
#[derive(Serialize, Deserialize, Debug)]
pub struct Peer {
    id: String,
    #[serde(rename = "type")]
    peer_type: PeerType,
    name: String,
    username: String,
    display_picture_sizes: Vec<DisplayPictureSize>,
    flags: Vec<String>,
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
    id: String,
    file_mime: String,
    file_name: String,
    file_size: u32,
    file_type: FileType,
    flags: Vec<String>,
    created_timestamp: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileType {
    DOCUMENT,
    PHOTO,
    VIDEO,
    AUDIO,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelpDocument {
    id: String,
    text: String,
    entities: Vec<TextEntity>,
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
    network_name: String,
    protocol_version: String,
    cdn_server: String,
    upload_max_file_size: u32,
    unauthorized_session_ttl: u32,
    authorized_session_ttl: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
