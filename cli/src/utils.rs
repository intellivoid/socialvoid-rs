use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};
use std::time::{Duration, SystemTime};

use crate::error::MyFriendlyError;
use socialvoid::session::SessionHolder;
use socialvoid_types::ServerInformation;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub session_file: String,
    pub base_path: String,
    pub config_path: String,
    pub cached_stuff_path: String,
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
            cached_stuff_path: format!("{}/cached_stuff", base_path),
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

pub async fn init_all(config: &Config) -> (socialvoid::Client, CachedStuff) {
    //TODO: only send errors from here and let the main.rs handle situations for panics

    //load cached stuff
    let mut cached: CachedStuff = match std::fs::read_to_string(&config.cached_stuff_path) {
        Ok(cached_stuff_str) => {
            serde_json::from_str(&cached_stuff_str).unwrap_or_else(|_| CachedStuff::default())
        }
        Err(_err) => CachedStuff::default(),
    };

    if let Ok(cache_last_updated) = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH + Duration::from_secs(cached.last_updated_on))
    {
        if cache_last_updated.as_secs() > cached.update_after {
            cached.update().await;
            match cached.save(&config.cached_stuff_path) {
                Ok(_) => {}
                Err(err) => println!("Error while saving cache.\n{}", err),
            }
        }
    } else {
        cached.update().await;
        match cached.save(&config.cached_stuff_path) {
            Ok(_) => {}
            Err(err) => println!("Error while saving cache.\n{}", err),
        }
    }

    //load sessions
    let client = match std::fs::read(&config.session_file) {
        Ok(bytes) => {
            match socialvoid::new(
                SessionHolder::deserialize(bytes),
                cached.rpc_url.clone(),
                cached
                    .server_info
                    .as_ref()
                    .map(|server_info| server_info.cdn_server.clone()),
            )
            .await
            {
                Ok(client) => client,
                Err(_) => panic!(
                "The session file may be corrupt. try deleting it to have a new session created."
                ),
            }
        }
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
    };

    (client, cached)
}

#[derive(Serialize, Deserialize)]
pub struct CachedStuff {
    /// unix timestamp when cache updated last time
    last_updated_on: u64,
    /// update cache if `update_after` seconds have passed after updating
    /// it the last time (since `last_updated_on`).
    update_after: u64,

    pub rpc_url: Option<String>, //only updated by the user

    pub server_info: Option<ServerInformation>,
}

impl Default for CachedStuff {
    fn default() -> Self {
        Self {
            last_updated_on: 0,
            update_after: 86400, // 1 day
            rpc_url: Some("http://socialvoid.qlg1.com:5601/".to_string()),
            server_info: None,
        }
    }
}

impl CachedStuff {
    async fn update(&mut self) {
        let sv = socialvoid::new_with_host(self.rpc_url.clone());
        if let Ok(server_info) = sv.help.get_server_information().await {
            self.server_info = Some(server_info);

            self.last_updated_on = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        } else {
            println!("There was a problem while updating the cache.");
        }
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, serde_json::to_string(&self).unwrap())?;
        Ok(())
    }
}
