use serde::{Deserialize, Serialize};

/// A Peer Object that contains information about the peer
#[derive(Serialize, Deserialize, Debug)]
pub struct Peer {
    pub id: String,
    #[serde(rename = "type")]
    pub peer_type: PeerType,
    pub name: String,
    pub username: String,
    pub display_picture_sizes: Vec<DisplayPictureSize>,
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
