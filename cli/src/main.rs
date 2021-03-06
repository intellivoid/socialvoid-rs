use socialvoid::session::RegisterRequest;
use socialvoid_rawclient::AuthenticationError;
use socialvoid_rawclient::ErrorKind;
use structopt::StructOpt;

mod entities;
mod error;
mod utils;
use crate::entities::*;
use crate::utils::*;
use error::MyFriendlyError;

// TODO: maybe try to remove ALL the expect calls and use only MyFriendlyError everywhere. + improve the MyFriendlyError
// TODO: add signal handling? + support for windows?
#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    let config = load_config();

    let (sv, mut cached) = init_all(&config).await;

    if let Some(cmd) = args.commands {
        match cmd {
            SocialVoidCommand::Login { username } => {
                if sv.session.authenticated() {
                    panic!(
                        "Already logged in. You should log out before logging into a new account."
                    )
                }

                let username = if let Some(username) = username {
                    username
                } else {
                    prompt_stdin("Your username on [network url here?]: ")
                };
                let password = prompt_password("Enter password: ");
                //TODO: add OTP support
                match sv
                    .session
                    .authenticate_user(username.clone(), password.clone(), None)
                    .await
                {
                    Err(err) => match err.kind {
                        ErrorKind::Authentication(AuthenticationError::SessionExpired) => {
                            println!("Session expired. Creating new session and retrying.");
                            match sv.session.create().await {
                                Ok(_) => {
                                    match sv
                                        .session
                                        .authenticate_user(username, password, None)
                                        .await
                                    {
                                        Err(err) => println!(
                                            "Couldn't authenticate the user.\n{}",
                                            MyFriendlyError::from(err)
                                        ),
                                        Ok(_) => println!("Done."),
                                    }
                                }
                                Err(err) => {
                                    println!(
                                        "Couldn't create the session.\n{}",
                                        MyFriendlyError::from(err)
                                    )
                                }
                            }
                        }
                        _ => println!(
                            "Couldn't authenticate the user.\n{}",
                            MyFriendlyError::from(err)
                        ),
                    },
                    Ok(_) => {
                        println!("Successfully logged in.");
                    }
                }
            }
            SocialVoidCommand::Logout => {
                let _ = sv.session.logout().await;
            }
            SocialVoidCommand::Register => {
                if sv.session.authenticated() {
                    panic!(
                        "Already logged in. You should log out before logging into a new account."
                    )
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
                    .help
                    .get_terms_of_service()
                    .await
                    .expect("Couldn't get the terms of service.");
                //TODO: find a better way to show this
                println!("{}", tos.get_plain_text());
                let accept_tos =
                    prompt_stdin("Have you read these terms of service and accept them?[y/N] ")
                        .chars()
                        .next();
                if let Some(accept_tos) = accept_tos {
                    if accept_tos == 'y' || accept_tos == 'Y' {
                        sv.session.accept_terms_of_service(tos);
                        match sv
                            .session
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
            SocialVoidCommand::Config { .. } => {
                println!("WIP");
            }
            SocialVoidCommand::GetMe => match sv.network.get_me().await {
                Ok(response) => println!("{:#?}", response),
                Err(err) => println!("{}", MyFriendlyError::from(err)),
            },
            SocialVoidCommand::Followers { peer, page } => {
                match sv.network.get_followers(peer, page).await {
                    Ok(peers) => {
                        println!(
                            "{}",
                            peers
                                .iter()
                                .map(|peer| format!("{:?}", peer))
                                .collect::<Vec<String>>()
                                .join("\n")
                        )
                    }
                    Err(err) => println!("{}", MyFriendlyError::from(err)),
                }
            }
            SocialVoidCommand::Following { peer, page } => {
                match sv.network.get_following(peer, page).await {
                    Ok(peers) => {
                        println!(
                            "{}",
                            peers
                                .iter()
                                .map(|peer| format!("{:?}", peer))
                                .collect::<Vec<String>>()
                                .join("\n")
                        )
                    }
                    Err(err) => println!("{}", MyFriendlyError::from(err)),
                }
            }
            SocialVoidCommand::SetProfile { field, value } => match field {
                ProfileField::Pic => {
                    if let Some(filepath) = value {
                        match sv.account.set_profile_picture(filepath).await {
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
            },
            SocialVoidCommand::Profile { peer } => match sv.network.get_profile(peer).await {
                Ok(profile) => println!("{}", SVProfile::from(profile)),
                Err(err) => println!(
                    "An error occurred while trying to get the profile.\n{}",
                    MyFriendlyError::from(err)
                ),
            },
            SocialVoidCommand::Follow { peer } => {
                match sv.network.follow_peer(peer.clone()).await {
                    Ok(relationship) => {
                        println!(
                            "The relationship type with\n\t`{}`\nis now\n\t{:?}",
                            peer, relationship
                        )
                    }
                    Err(err) => println!("{}", MyFriendlyError::from(err)),
                }
            }
            SocialVoidCommand::Unfollow { peer } => {
                match sv.network.unfollow_peer(peer.clone()).await {
                    Ok(relationship) => {
                        println!(
                            "The relationship type with\n\t`{}`\nis now\n\t{:?}",
                            peer, relationship
                        )
                    }
                    Err(err) => println!("{}", MyFriendlyError::from(err)),
                }
            }
            SocialVoidCommand::Feed { page } => match sv.timeline.retrieve_feed(page).await {
                Ok(feed) => {
                    let mut pager = minus::Pager::new().unwrap();

                    let n_posts = feed.len();
                    for post in feed.into_iter() {
                        let post = SVPost::from(post);
                        pager.push_str(format!("================\n{}", post));
                    }
                    pager.push_str(format!(
                        "----Retrieved {} post(s) from the timeline.\n",
                        n_posts
                    ));
                    pager.set_prompt("Feed - Socialvoid");
                    minus::page_all(pager).expect("Error with pager");
                }
                Err(err) => println!("{}", MyFriendlyError::from(err)),
            },
            SocialVoidCommand::Like { post_id } => match sv.timeline.like(post_id).await {
                Ok(_) => println!("Done"),
                Err(err) => println!("{}", MyFriendlyError::from(err)),
            },
            SocialVoidCommand::Unlike { post_id } => match sv.timeline.unlike(post_id).await {
                Ok(_) => println!("Done"),
                Err(err) => println!("{}", MyFriendlyError::from(err)),
            },
            SocialVoidCommand::GetPost { post_id } => match sv.timeline.get_post(post_id).await {
                Ok(post) => println!("{}", SVPost::from(post)),
                Err(err) => println!("{}", MyFriendlyError::from(err)),
            },
            SocialVoidCommand::DeletePost { post_id } => match sv.timeline.delete(post_id).await {
                Ok(_ok) => println!("Done"),
                Err(err) => println!("{}", MyFriendlyError::from(err)),
            },
            SocialVoidCommand::Sync {} => {}
        }
    }

    // todo: maybe save these again ONLY if modified
    cached
        .save(&config.cached_stuff_path)
        .expect("Couldn't save cached stuff");
    std::fs::write(&config.session_file, sv.session.serialize()).unwrap();
    save_config(&config).expect("Couldn't save the config");
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
    Logout,
    Register,
    Config {
        #[structopt(subcommand)]
        field: ConfigField,
        value: Option<String>,
    },
    GetMe,
    Followers {
        peer: Option<String>,
        page: Option<u32>,
    },
    Following {
        peer: Option<String>,
        page: Option<u32>,
    },
    Follow {
        peer: String,
    },
    Unfollow {
        peer: String,
    },
    SetProfile {
        #[structopt(subcommand)]
        field: ProfileField,
        value: Option<String>,
    },
    Profile {
        peer: Option<String>,
    },
    Feed {
        page: Option<u32>,
    },
    GetPost {
        post_id: String,
    },
    Like {
        post_id: String,
    },
    Unlike {
        post_id: String,
    },
    DeletePost {
        post_id: String,
    },
    Sync {},
}

#[derive(Debug, StructOpt)]
enum ProfileField {
    Pic, //TODO: remove this and make a separate command for profile pic
}

#[derive(Debug, StructOpt)]
enum ConfigField {
    Sessions,
}
