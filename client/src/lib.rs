pub mod account;
pub mod help;
pub mod network;
pub mod session;

use session::ClientInfo;
use session::RegisterRequest;
use session::Session;
use session::SessionHolder;
use socialvoid_rawclient::Error;
use socialvoid_types::Document;
use socialvoid_types::HelpDocument;
use socialvoid_types::Peer;

/// Create a client and establish a new session
pub async fn new_with_defaults() -> Result<Client, Error> {
    let rpc_client = socialvoid_rawclient::new();
    let cdn_client = make_cdn_client_from(&rpc_client).await?;
    let client_info = ClientInfo::generate();
    let mut session = SessionHolder::new(client_info.clone());
    session.create(&rpc_client).await?;
    let sessions = vec![session];

    Ok(Client {
        sessions,
        client_info,
        rpc_client,
        cdn_client,
    })
}

/// Creates the CDN client by resolving the host url from server information
async fn make_cdn_client_from(
    rpc_client: &socialvoid_rawclient::Client,
) -> Result<socialvoid_rawclient::CdnClient, Error> {
    let server_info = help::get_server_information(&rpc_client).await?;

    Ok(socialvoid_rawclient::CdnClient::with_cdn_url(
        server_info.cdn_server,
    ))
}

/// Create a client with user defined client info and sessions
/// And CDN as gven in the server information
/// TODO: maybe verify the session and return an error if session is invalid
pub async fn new(client_info: ClientInfo, sessions: Vec<SessionHolder>) -> Result<Client, Error> {
    let rpc_client = socialvoid_rawclient::new();
    let cdn_client = make_cdn_client_from(&rpc_client).await?;
    Ok(Client {
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
        sessions: Vec::new(),
        client_info: ClientInfo::generate(),
        rpc_client: socialvoid_rawclient::new(),
        cdn_client: socialvoid_rawclient::CdnClient::new(),
    }
}

/// A client that can be used to call methods and manage sessions for Social Void
pub struct Client {
    pub sessions: Vec<SessionHolder>,
    client_info: ClientInfo,
    rpc_client: socialvoid_rawclient::Client,
    cdn_client: socialvoid_rawclient::CdnClient,
}

impl Client {
    /// Set the CDN server URL from the ServerInfomation
    pub async fn reset_cdn_url(&mut self) -> Result<(), Error> {
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
        self.sessions.extend(sessions);
        Ok(())
    }

    /// Tries to establish another session adds it to the client if successful and returns the key of the session
    pub async fn new_session(&mut self) -> Result<usize, Error> {
        let mut session = SessionHolder::new(self.client_info.clone());
        session.create(&self.rpc_client).await?;
        self.sessions.push(session);

        Ok(self.sessions.len() - 1)
    }

    /// Removes a session and returns it
    pub async fn delete_session(&mut self, session_key: usize) -> SessionHolder {
        self.sessions.remove(session_key)
    }

    /// Gets a Session object for a specific session
    pub async fn get_session(&mut self, session_key: usize) -> Result<Session, Error> {
        self.sessions[session_key].get(&self.rpc_client).await
    }

    /// Get terms of service
    pub async fn get_terms_of_service(&self) -> Result<HelpDocument, Error> {
        help::get_terms_of_service(&self.rpc_client).await
    }

    /// Accept terms of service for a specific session
    pub fn accept_tos(&mut self, session_key: usize, tos: HelpDocument) {
        self.sessions[session_key].accept_terms_of_service(tos);
    }

    /// Register an account using a specific session
    pub async fn register(
        &mut self,
        session_key: usize,
        req: RegisterRequest,
    ) -> Result<Peer, Error> {
        self.sessions[session_key]
            .register(req, &self.rpc_client)
            .await
    }

    /// Login to an account using a specific session
    pub async fn authenticate_user(
        &mut self,
        session_key: usize,
        username: String,
        password: String,
        otp: Option<String>,
    ) -> Result<bool, Error> {
        self.sessions[session_key]
            .authenticate_user(&self.rpc_client, username, password, otp)
            .await
    }

    /// Check if a session is authenticated
    pub fn is_authenticated(&self, session_key: usize) -> bool {
        self.sessions[session_key].authenticated()
    }

    /// Log out from a session. Maybe destroy the session??
    pub async fn logout(&mut self, session_key: usize) -> Result<bool, Error> {
        self.sessions[session_key].logout(&self.rpc_client).await
    }

    pub async fn get_me(&self, session_key: usize) -> Result<Peer, Error> {
        network::get_me(
            &self.rpc_client,
            self.sessions[session_key].session_identification()?,
        )
        .await
    }

    pub async fn set_profile_picture(
        &self,
        session_key: usize,
        filepath: String,
    ) -> Result<Document, Error> {
        let sesh_id = self.sessions[session_key].session_identification()?;
        let document = self.cdn_client.upload(sesh_id.clone(), filepath).await?;
        account::set_profile_picture(&self.rpc_client, sesh_id, document.id.clone()).await;
        Ok(document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_get_a_session() {}

    #[tokio::test]
    async fn it_should_log_in_and_get_the_correct_peer() -> Result<(), Error> {
        let sessions_file = "sessions.test";

        let creds: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string("test_creds.test").unwrap())?;

        let mut client = new_empty_client();
        match client.load_sessions(sessions_file) {
            Err(_) => {
                client.new_session().await?;

                client
                    .authenticate_user(
                        0,
                        creds["username"].as_str().unwrap().to_string(),
                        creds["password"].as_str().unwrap().to_string(),
                        None,
                    )
                    .await?;
                client.save_sessions(sessions_file).unwrap();
            }
            _ => {}
        }

        let peer = client.get_me(0).await?;
        client.logout(0).await?;

        println!("{:?}", peer);
        assert_eq!(
            peer.username,
            creds["username"].as_str().unwrap().to_string()
        );

        Ok(())
    }
}
