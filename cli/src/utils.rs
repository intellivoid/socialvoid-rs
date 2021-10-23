use rpassword::read_password;
use serde::{Deserialize, Serialize};
use socialvoid as sv_client;
use socialvoid::SocialvoidError;
use socialvoid_rawclient::{AuthenticationError, ErrorKind};
use std::io::{stdin, stdout, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub sessions_file: String,
    pub base_path: String,
    pub config_path: String,
    pub current_session: usize,
}

pub async fn setup_sessions(config: &Config, sv: &mut sv_client::Client, sesh_key: &mut usize) {
    if let Err(err) = sv.load_sessions(&config.sessions_file) {
        if err.kind() == std::io::ErrorKind::NotFound {
            sv.new_session()
                .await
                .expect("Couldn't create a new session.");
        } else {
            panic!("Couldn't load sessions from the config\n{:?}", err);
        }
    }

    if sv.sessions.len() == 0 {
        sv.new_session()
            .await
            .expect("Couldn't create a new session.");
    }

    match sv.get_session(*sesh_key).await {
        Ok(_) => {}
        Err(err) => match err {
            SocialvoidError::RawClient(err) => match err.kind {
                ErrorKind::Authentication(AuthenticationError::SessionExpired)
                | ErrorKind::Authentication(AuthenticationError::SessionNotFound) => {
                    println!("This session either didn't exist or is expired.\nDeleting it and creating a new one.");
                    sv.delete_session(*sesh_key).await;
                    let new_sesh_key = sv
                        .new_session()
                        .await
                        .expect("Couldn't create a new session.");
                    *sesh_key = new_sesh_key;
                }
                _ => {
                    panic!(
                        "Couldn't `get` session. The session is probably corrupt.
Either delete the sessions file or fix the corrupt session."
                    );
                }
            },
            _ => {
                panic!(
                    "Couldn't `get` session. The session is probably corrupt.
Either delete the sessions file or fix the corrupt session."
                );
            }
        },
    }
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
            sessions_file: format!("{}/sessions", base_path),
            base_path,
            config_path,
            current_session: 0,
        },
    }
}

pub fn save_config(sesh_key: usize, config: &Config) -> Result<(), std::io::Error> {
    let mut config = config.clone();
    config.current_session = sesh_key;
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
