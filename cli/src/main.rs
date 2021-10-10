use client as sv_client;
use structopt::StructOpt;

mod utils;
use crate::utils::*;

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
            setup_sessions(&config, &mut sv, &mut current_session).await;
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
            let password = prompt_password("Enter password: ");
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
            setup_sessions(&config, &mut sv, &mut current_session).await;

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

#[derive(Debug, StructOpt)]
enum Cli {
    Login { username: Option<String> },
    Config { server: Option<usize> },
    GetMe,
    Sync {},
}
