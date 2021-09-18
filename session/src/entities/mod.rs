mod client_info;
pub use client_info::ClientInfo;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionEstablished {
    pub id: String,
    pub challenge: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionIdentification {
    session_id: String,
    client_public_hash: String,
    challenge_answer: String,
}
