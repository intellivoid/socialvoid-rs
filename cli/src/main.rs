use socialvoid as sv_client;
use socialvoid::session::RegisterRequest;
use structopt::StructOpt;

mod error;
mod utils;
use crate::utils::*;
use error::MyFriendlyError;

// TODO: maybe try to remove ALL the expect calls and use only MyFriendlyError everywhere. + improve the MyFriendlyError
// TODO: add signal handling? + support for windows?
#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    let config = load_config();
    let mut sv = sv_client::new_empty_client();
    sv.reset_cdn_url().await.unwrap(); //set proper CDN url
    let mut current_session: usize = std::env::var("SV_CURRENT_SESSION")
        .unwrap_or_else(|_| config.current_session.to_string())
        .parse()
        .expect("The environment variable should SV_CURRENT_SESSION should contain an integer >=0");

    if let Some(cmd) = args.commands {
        match cmd {
            SocialVoidCommand::Login { username } => {
                setup_sessions(&config, &mut sv, &mut current_session).await;
                let username = if let Some(username) = username {
                    username
                } else {
                    prompt_stdin("Your username on [network url here?]: ")
                };
                let sk = if sv.is_authenticated().unwrap() {
                    sv.new_session()
                        .await
                        .expect("Couldn't create a new session.")
                } else {
                    current_session
                };
                current_session = sk;
                sv.set_current_session(current_session).unwrap();
                let password = prompt_password("Enter password: ");
                //TODO: add OTP support
                match sv.authenticate_user(username, password, None).await {
                    Err(err) => {
                        println!(
                            "Couldn't authenticate the user.\n{}",
                            MyFriendlyError::from(err)
                        );
                    }
                    Ok(_) => {
                        println!("Successfully logged in.");
                    }
                }
            }
            SocialVoidCommand::Register => {
                setup_sessions(&config, &mut sv, &mut current_session).await;
                if sv.is_authenticated().unwrap() {
                    current_session = sv
                        .new_session()
                        .await
                        .expect("Couldn't create a new session");
                    sv.set_current_session(current_session).unwrap();
                }
                let first_name = prompt_stdin("First name: ");
                let last_name = {
                    let last_name = prompt_stdin("Last name(optional): ");
                    if last_name.is_empty() {
                        None
                    } else {
                        Some(last_name)
                    }
                };
                //TODO: don't login if already logged in for a username???

                let username = prompt_stdin("New username: ");
                let password = loop {
                    let password = prompt_password("Enter password: ");
                    let confirm_password = prompt_password("Enter password again: ");
                    if password == confirm_password {
                        break password;
                    } else {
                        //TODO: validation and Maybe password strength check or sth ^^
                        println!("Password don't match. Please try again or Ctrl+C to quit.");
                    }
                };
                let tos = sv
                    .get_terms_of_service()
                    .await
                    .expect("Couldn't get the terms of service.");
                println!("{}", tos.get_plain_text());
                let accept_tos =
                    prompt_stdin("Have you read these terms of service and accept them?[y/N] ")
                        .chars()
                        .next();
                if let Some(accept_tos) = accept_tos {
                    if accept_tos == 'y' || accept_tos == 'Y' {
                        sv.accept_tos(tos).unwrap();
                        match sv
                            .register(RegisterRequest {
                                username,
                                password,
                                first_name,
                                last_name,
                            })
                            .await
                        {
                            Ok(peer) => println!("Registered.\n{:#?}", peer),
                            Err(err) => {
                                println!("Couldn't register.\n{}", MyFriendlyError::from(err))
                            }
                        }
                    } else {
                        println!(
                            "You need to accept the terms of service to register to SocialVoid"
                        );
                    }
                } else {
                    println!("You need to accept the terms of service to register to SocialVoid");
                }
            }
            SocialVoidCommand::Config { field, value } => {
                match field {
                    ConfigField::Sessions => {
                        setup_sessions(&config, &mut sv, &mut current_session).await;
                        //Gets the sessions if value is none, other wise sets to a session key that is valid
                        if let Some(sesh_key) = value {
                            if let Ok(sesh_key) = sesh_key.parse::<usize>() {
                                match sv.set_current_session(sesh_key) {
                                    Ok(_) => {
                                        current_session = sesh_key;
                                        println!("Changed session to {}\n", sesh_key);
                                    }
                                    Err(err) => println!("{}", MyFriendlyError::from(err)),
                                }
                            } else {
                                println!("Enter the session index for it to switch to a session");
                                println!("There are {} sessions.", sv.sessions.len());
                            }
                        } else {
                            println!(
                                "There are {} session(s).\nCurrent session: {}",
                                sv.sessions.len(),
                                current_session
                            );
                        }
                    }
                }
            }
            SocialVoidCommand::GetMe => {
                setup_sessions(&config, &mut sv, &mut current_session).await;

                match sv.get_me().await {
                    Ok(response) => println!("{:#?}", response),
                    Err(err) => println!("{}", MyFriendlyError::from(err)),
                }
            }
            SocialVoidCommand::SetProfile { field, value } => {
                setup_sessions(&config, &mut sv, &mut current_session).await;
                match field {
                    ProfileField::Pic => {
                        if let Some(filepath) = value {
                            match sv.set_profile_picture(filepath).await {
                                Ok(doc) => {
                                    println!("Profile picture updated successfully.\n{:?}", doc);
                                }
                                Err(err) => {
                                    println!(
                                        "An error occurred while setting the profile picture.\n{}",
                                        MyFriendlyError::from(err)
                                    );
                                }
                            }
                        } else {
                            println!("You need to specify the path to the picture to upload");
                        }
                    }
                }
            }
            SocialVoidCommand::GetProfile { field } => {
                setup_sessions(&config, &mut sv, &mut current_session).await;
                match field {
                    Some(field) => match field {
                        ProfileField::Pic => {
                            let _filepath
                            = prompt_stdin("Where would you like to save the profile picture(default: TODO: show path of default directory)?");
                            unimplemented!()
                        }
                    },
                    None => {
                        // The full profile
                        match sv.get_my_profile().await {
                            Ok(profile) => println!("{}", profile),
                            Err(err) => println!(
                                "An error occurred while trying to get the profile.\n{}",
                                MyFriendlyError::from(err)
                            ),
                        }
                    }
                }
            }
            SocialVoidCommand::Sync {} => {}
        }
    }

    sv.save_sessions(&config.sessions_file)
        .expect("Couldn't save the sessions");
    save_config(current_session, &config).expect("Couldn't save the config");
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    commands: Option<SocialVoidCommand>,
}

#[derive(Debug, StructOpt)]
enum SocialVoidCommand {
    Login {
        username: Option<String>,
    },
    Register,
    Config {
        #[structopt(subcommand)]
        field: ConfigField,
        value: Option<String>,
    },
    GetMe,
    SetProfile {
        #[structopt(subcommand)]
        field: ProfileField,
        value: Option<String>,
    },
    GetProfile {
        #[structopt(subcommand)]
        field: Option<ProfileField>,
    },
    Sync {},
}

#[derive(Debug, StructOpt)]
enum ProfileField {
    Pic,
}

#[derive(Debug, StructOpt)]
enum ConfigField {
    Sessions,
}
