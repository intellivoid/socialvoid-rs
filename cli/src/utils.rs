use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};

use crate::error::MyFriendlyError;
use socialvoid::session::SessionHolder;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub session_file: String,
    pub base_path: String,
    pub config_path: String,
}

pub fn load_config() -> Config {
    //TODO: Use config from a file
    let base_path = match std::env::var("SV_CLI_PATH") {
        Ok(val) => shellexpand::full(&val).unwrap().to_string(),
        Err(_) => {
            let se = shellexpand::tilde("~/.sv-cli").to_string();
            std::fs::create_dir_all(&se).expect("Failed to create directory at ~/.sv-cli");
            se
        }
    };

    let config_path = format!("{}/config.json", base_path);
    match std::fs::read_to_string(&config_path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(data) => data,
            Err(err) => {
                panic!("Error processing JSON in config file.\n{:?}", err)
            }
        },
        Err(_err) => Config {
            session_file: format!("{}/session", base_path),
            base_path,
            config_path,
        },
    }
}

pub fn save_config(config: &Config) -> Result<(), std::io::Error> {
    // let mut config = config.clone();
    std::fs::write(&config.config_path, serde_json::to_string(&config).unwrap())?;
    Ok(())
}

pub fn prompt_stdin(prompt: &str) -> String {
    print!("{}", prompt);
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Couldn't read string"); //TODO: validation?
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

pub fn prompt_password(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    read_password().expect("Couldn't read the password")
}

pub async fn init_client(config: &Config) -> socialvoid::Client {
    match std::fs::read(&config.session_file) {
        Ok(bytes) => match socialvoid::new(SessionHolder::deserialize(bytes)).await {
            Ok(client) => client,
            Err(_) => panic!(
                "The session file may be corrupt. try deleting it to have a new session created."
            ),
        },
        Err(err) => {
            println!(
                "There was a problem while reading the session file.\n{}",
                err
            );
            // TODO: give the user the option to either quit or to change the path of the session file
            // also look into taking the path of the config and session from the command line
            println!("Creating new session.");
            match socialvoid::new_with_defaults().await {
                Ok(client) => client,
                Err(err) => panic!(
                    "There was an error while trying to establish a new session.\n{}",
                    MyFriendlyError::from(err)
                ),
            }
        }
    }
}
