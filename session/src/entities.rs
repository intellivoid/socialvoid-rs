use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionEstablished {
    id: String,
    challenge: String,
}

#[derive(Serialize)]
pub struct ClientInfo {
    pub public_hash: String,
    pub private_hash: String,
    pub name: String,
    pub platform: String,
    pub version: String,
}

impl ClientInfo {
    ///Generates client information
    pub fn generate() -> ClientInfo {
        let public_hash = generate_random_hash();
        let private_hash = generate_random_hash();
        let platform = String::from("Linux"); //TODO: auto detect platform?
        let name = String::from("Social Void rust");
        let version = String::from("0.0.1"); //maybe have a better way to set this?
        ClientInfo {
            public_hash,
            private_hash,
            platform,
            name,
            version,
        }
    }
}

fn generate_random_hash() -> String {
    //TODO: make it secure, more random idk
    sha256::digest::<String>(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect(),
    )
}
