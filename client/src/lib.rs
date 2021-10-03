use rawclient::Error;
use session::ClientInfo;
use session::RegisterRequest;
use session::SessionHolder;
use types::HelpDocument;
use types::Peer;

/// Create a client and establish a new session
pub async fn new_with_defaults() -> Result<Client, Error> {
    let rpc_client = rawclient::new();
    let client_info = ClientInfo::generate();
    let mut session = SessionHolder::new(client_info.clone());
    session.create(&rpc_client).await?;
    let sessions = vec![session];

    Ok(Client {
        sessions,
        client_info,
        rpc_client,
    })
}

/// Create a client with user defined client info and sessions
/// TODO: maybe verify the session and return an error if session is invalid
pub fn new(client_info: ClientInfo, sessions: Vec<SessionHolder>) -> Result<Client, Error> {
    let rpc_client = rawclient::new();
    Ok(Client {
        sessions,
        client_info,
        rpc_client,
    })
}

/// Create a client with generated client info and zero sessions
pub fn new_empty_client() -> Client {
    Client {
        sessions: Vec::new(),
        client_info: ClientInfo::generate(),
        rpc_client: rawclient::new(),
    }
}

/// A client that can be used to call methods and manage sessions for Social Void
pub struct Client {
    pub sessions: Vec<SessionHolder>,
    client_info: ClientInfo,
    rpc_client: rawclient::Client,
}

impl Client {
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
        &self,
        session_key: usize,
        username: String,
        password: String,
        otp: Option<String>,
    ) -> Result<bool, Error> {
        self.sessions[session_key]
            .authenticate_user(&self.rpc_client, username, password, otp)
            .await
    }

    /// Log out from a session. Maybe destroy the session??
    pub async fn logout(&self, session_key: usize) -> Result<bool, Error> {
        self.sessions[session_key].logout(&self.rpc_client).await
    }

    pub async fn get_me(&self, session_key: usize) -> Result<Peer, Error> {
        network::get_me(
            &self.rpc_client,
            self.sessions[session_key].session_identification()?,
        )
        .await
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
