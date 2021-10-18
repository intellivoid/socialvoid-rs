use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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
        let platform = env::consts::OS.to_string(); //String::from("Linux"); //TODO: auto detect platform?
        let name = String::from("Social Void Rust");
        let version = String::from("0.0.1"); //maybe have a better way to set this?
        ClientInfo {
            public_hash,
            private_hash,
            platform,
            name,
            version,
        }
    }

    pub fn save(&self, fpath: &str) -> Result<(), std::io::Error> {
        serde_json::to_writer(&std::fs::File::create(&fpath)?, self)?;
        Ok(())
    }

    pub fn load_from_file(fpath: &str) -> Result<ClientInfo, std::io::Error> {
        let client_info: ClientInfo = serde_json::from_reader(&std::fs::File::open(fpath)?)?;

        Ok(client_info)
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
