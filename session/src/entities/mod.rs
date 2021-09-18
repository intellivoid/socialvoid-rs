mod client_info;
pub use client_info::ClientInfo;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionEstablished {
    id: String,
    challenge: String,
}
