use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};

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
