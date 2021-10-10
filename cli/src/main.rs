use client as sv_client;
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    let config = load_config();
    let mut sv = sv_client::new_empty_client();
    let mut current_session: usize = std::env::var("SV_CURRENT_SESSION")
        .unwrap_or("0".to_string())
        .parse()
        .expect("The environment variable should SV_CURRENT_SESSION should contain an integer >=0");

    match args {
        Cli::Login { username } => {
            load_session_and_new_session_if_notfound_or_panic(&config, &mut sv).await;
            let username = if let Some(username) = username {
                username
            } else {
                prompt_stdin("Your username on [network url here?]: ")
            };
            let sk = if sv.is_authenticated(current_session) {
                sv.new_session()
                    .await
                    .expect("Couldn't create a new session.")
            } else {
                current_session
            };
            current_session = sk;
            // maybe have a way to check if username already exists ??
            let password = prompt_stdin("Enter password: ");
            //TODO: add OTP support
            match sv.authenticate_user(sk, username, password, None).await {
                Err(err) => {
                    println!("Couldn't authenticate the user.\n{:#?}", err);
                }
                Ok(_) => {
                    println!("Successfully logged in.");
                }
            }
        }
        Cli::Config { .. } => {}
        Cli::GetMe => {
            load_session_and_new_session_if_notfound_or_panic(&config, &mut sv).await;

            match sv.get_me(current_session).await {
                Ok(response) => println!("{:?}", response),
                Err(err) => println!("{:?}", err),
            }
        }
        Cli::Sync {} => {}
    }

    sv.save_sessions(&config.sessions_file)
        .expect("Couldn't save the sessions");
    save_config(current_session, &config).expect("Couldn't save the config");
}

async fn load_session_and_new_session_if_notfound_or_panic(
    config: &Config,
    sv: &mut sv_client::Client,
) {
    if let Err(err) = sv.load_sessions(&config.sessions_file) {
        if err.kind() == std::io::ErrorKind::NotFound {
            sv.new_session()
                .await
                .expect("Couldn't create a new session.");
        } else {
            panic!("Couldn't load sessions from the config\n{:?}", err);
        }
    }
}

#[derive(Debug, StructOpt)]
enum Cli {
    Login { username: Option<String> },
    Config { server: Option<usize> },
    GetMe,
    Sync {},
}

fn load_config() -> Config {
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
            base_path: base_path,
            config_path,
            current_session: 0,
        },
    }
}

fn save_config(sesh_key: usize, config: &Config) -> Result<(), std::io::Error> {
    let mut config = config.clone();
    config.current_session = sesh_key;
    std::fs::write(&config.config_path, serde_json::to_string(&config).unwrap())?;
    Ok(())
}

fn prompt_stdin(prompt: &str) -> String {
    print!("{}", prompt);
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a valid username"); //TODO: validation?
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    sessions_file: String,
    base_path: String,
    config_path: String,
    current_session: usize,
}
