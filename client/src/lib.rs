pub mod account;
pub mod error;
pub mod help;
pub mod network;
pub mod session;

pub use error::ClientError;
pub use error::SocialvoidError;
use session::ClientInfo;
use session::RegisterRequest;
use session::Session;
use session::SessionHolder;
use socialvoid_types::Document;
use socialvoid_types::HelpDocument;
use socialvoid_types::Peer;
use socialvoid_types::Profile;

/// Create a client and establish a new session
pub async fn new_with_defaults() -> Result<Client, SocialvoidError> {
    let rpc_client = socialvoid_rawclient::new();
    let cdn_client = make_cdn_client_from(&rpc_client).await?;
    let client_info = ClientInfo::generate();
    let mut session = SessionHolder::new(client_info.clone());
    session.create(&rpc_client).await?;
    let sessions = vec![session];

    Ok(Client {
        current_session: Some(0),
        sessions,
        client_info,
        rpc_client,
        cdn_client,
    })
}

/// Creates the CDN client by resolving the host url from server information
async fn make_cdn_client_from(
    rpc_client: &socialvoid_rawclient::Client,
) -> Result<socialvoid_rawclient::CdnClient, SocialvoidError> {
    let server_info = help::get_server_information(rpc_client).await?;

    Ok(socialvoid_rawclient::CdnClient::with_cdn_url(
        server_info.cdn_server,
    ))
}

/// Create a client with user defined client info and sessions
/// And CDN as gven in the server information
/// TODO: maybe verify the session and return an error if session is invalid
pub async fn new(
    client_info: ClientInfo,
    sessions: Vec<SessionHolder>,
) -> Result<Client, SocialvoidError> {
    let rpc_client = socialvoid_rawclient::new();
    let cdn_client = make_cdn_client_from(&rpc_client).await?;
    let current_session = if sessions.is_empty() { None } else { Some(0) };
    Ok(Client {
        current_session,
        sessions,
        client_info,
        rpc_client,
        cdn_client,
    })
}

/// Create a client with generated client info and zero sessions
/// Note that, cdn client may not be the one taken from server information
pub fn new_empty_client() -> Client {
    Client {
        current_session: None,
        sessions: Vec::new(),
        client_info: ClientInfo::generate(),
        rpc_client: socialvoid_rawclient::new(),
        cdn_client: socialvoid_rawclient::CdnClient::new(),
    }
}

/// A client that can be used to call methods and manage sessions for Social Void
pub struct Client {
    pub sessions: Vec<SessionHolder>,
    current_session: Option<usize>, //Index of the current session
    client_info: ClientInfo,
    rpc_client: socialvoid_rawclient::Client,
    cdn_client: socialvoid_rawclient::CdnClient,
}

impl Client {
    /// Set the CDN server URL from the ServerInfomation
    pub async fn reset_cdn_url(&mut self) -> Result<(), SocialvoidError> {
        self.cdn_client = make_cdn_client_from(&self.rpc_client).await?;
        Ok(())
    }

    /// Saves all your sessions to a file
    pub fn save_sessions(&self, filename: &str) -> Result<(), std::io::Error> {
        // let filename = "social-void-rust.sessions";
        serde_json::to_writer(&std::fs::File::create(filename)?, &self.sessions)?;
        Ok(())
    }

    /// Loads all sessions from a file and adds them to the client
    pub fn load_sessions(&mut self, fpath: &str) -> Result<(), std::io::Error> {
        let sessions: Vec<SessionHolder> = serde_json::from_reader(&std::fs::File::open(fpath)?)?;
        if self.sessions.is_empty() && !sessions.is_empty() {
            self.current_session = Some(0);
        }
        self.sessions.extend(sessions);
        Ok(())
    }

    /// Get another video

    /// Tries to establish another session adds it to the client if successful and returns the key of the session
    pub async fn new_session(&mut self) -> Result<usize, SocialvoidError> {
        let mut session = SessionHolder::new(self.client_info.clone());
        session.create(&self.rpc_client).await?;
        self.sessions.push(session);

        Ok(self.sessions.len() - 1)
    }

    /// Removes the current session and returns it
    pub fn delete_session(&mut self) -> Result<SessionHolder, SocialvoidError> {
        if self.current_session.is_none() {
            Err(SocialvoidError::Client(ClientError::NoSessionsExist))
        } else {
            let sesh_key = self.current_session.unwrap();
            self.current_session = if sesh_key == self.sessions.len() - 1 {
                if sesh_key != 0 {
                    Some(sesh_key - 1)
                } else {
                    None
                }
            } else {
                Some(sesh_key)
            };
            Ok(self.sessions.remove(sesh_key))
        }
    }

    /// Set the current session to session_key if exists
    pub fn set_current_session(&mut self, session_key: usize) -> Result<(), SocialvoidError> {
        if self.sessions.len() > session_key {
            self.current_session = Some(session_key);
            Ok(())
        } else {
            Err(SocialvoidError::Client(
                ClientError::SessionIndexOutOfBounds {
                    session_count: self.sessions.len(),
                },
            ))
        }
    }

    /// Get the current session key
    pub fn get_current_session_key(&self) -> Option<usize> {
        self.current_session.clone()
    }

    /// Gets a Session object for the current session
    pub async fn get_session(&mut self) -> Result<Session, SocialvoidError> {
        match self.current_session {
            Some(session_key) => Ok(self.sessions[session_key].get(&self.rpc_client).await?),
            None => Err(SocialvoidError::Client(ClientError::NoSessionsExist)),
        }
    }

    /// Get terms of service
    pub async fn get_terms_of_service(&self) -> Result<HelpDocument, SocialvoidError> {
        Ok(help::get_terms_of_service(&self.rpc_client).await?)
    }

    /// Accept terms of service for the current session
    pub fn accept_tos(&mut self, tos: HelpDocument) -> Result<(), SocialvoidError> {
        match self.current_session {
            Some(session_key) => {
                self.sessions[session_key].accept_terms_of_service(tos);
                Ok(())
            }
            None => Err(SocialvoidError::Client(ClientError::NoSessionsExist)),
        }
    }

    /// Register an account using the current session
    pub async fn register(&mut self, req: RegisterRequest) -> Result<Peer, SocialvoidError> {
        match self.current_session {
            Some(session_key) => Ok(self.sessions[session_key]
                .register(req, &self.rpc_client)
                .await?),
            None => Err(SocialvoidError::Client(ClientError::NoSessionsExist)),
        }
    }

    /// Login to an account using the current session
    pub async fn authenticate_user(
        &mut self,
        username: String,
        password: String,
        otp: Option<String>,
    ) -> Result<bool, SocialvoidError> {
        match self.current_session {
            Some(session_key) => Ok(self.sessions[session_key]
                .authenticate_user(&self.rpc_client, username, password, otp)
                .await?),
            None => Err(SocialvoidError::Client(ClientError::NoSessionsExist)),
        }
    }

    /// Check if current session is authenticated
    pub fn is_authenticated(&self) -> Result<bool, SocialvoidError> {
        match self.current_session {
            Some(session_key) => Ok(self.sessions[session_key].authenticated()),
            None => Err(SocialvoidError::Client(ClientError::NoSessionsExist)),
        }
    }

    /// Log out from the current session
    pub async fn logout(&mut self) -> Result<bool, SocialvoidError> {
        match self.current_session {
            Some(session_key) => {
                let log_out_resp = self.sessions[session_key].logout(&self.rpc_client).await?;
                self.delete_session()?;
                Ok(log_out_resp)
            }
            None => Err(SocialvoidError::Client(ClientError::NoSessionsExist)),
        }
    }

    /// Get Peer object of the authenticated user on the current session.
    pub async fn get_me(&self) -> Result<Peer, SocialvoidError> {
        match self.current_session {
            Some(session_key) => Ok(network::get_me(
                &self.rpc_client,
                self.sessions[session_key].session_identification()?,
            )
            .await?),
            None => Err(SocialvoidError::Client(ClientError::NoSessionsExist)),
        }
    }

    /// Get the profile of the authenticated user on the current session
    pub async fn get_my_profile(&self) -> Result<Profile, SocialvoidError> {
        match self.current_session {
            Some(session_key) => Ok(network::get_profile(
                &self.rpc_client,
                self.sessions[session_key].session_identification()?,
                None,
            )
            .await?),
            None => Err(SocialvoidError::Client(ClientError::NoSessionsExist)),
        }
    }

    /// Set the profile picture of the user on current session
    pub async fn set_profile_picture(&self, filepath: String) -> Result<Document, SocialvoidError> {
        match self.current_session {
            Some(session_key) => {
                let sesh_id = self.sessions[session_key].session_identification()?;
                let document = self.cdn_client.upload(sesh_id.clone(), filepath).await?;
                account::set_profile_picture(&self.rpc_client, sesh_id, document.id.clone())
                    .await?; //TODO: use result and send client error if false
                Ok(document)
            }
            None => Err(SocialvoidError::Client(ClientError::NoSessionsExist)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_should_log_in_and_get_the_correct_peer() -> Result<(), SocialvoidError> {
        // let sessions_file = "sessions.test";

        let creds: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string("test_creds.test").unwrap())?;

        let mut client = new_with_defaults().await?;
        client
            .authenticate_user(
                creds["username"].as_str().unwrap().to_string(),
                creds["password"].as_str().unwrap().to_string(),
                None,
            )
            .await?;

        let peer = client.get_me().await?;

        println!("{:?}", peer);
        client.logout().await?;
        assert_eq!(
            peer.username,
            creds["username"].as_str().unwrap().to_string()
        );

        Ok(())
    }
}
