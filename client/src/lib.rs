use rawclient::Error;
use session::ClientInfo;
use session::SessionEstablished;

/// Create a client and establish a new session
pub async fn new_with_defaults() -> Result<Client, Error> {
    let rpc_client = rawclient::new();
    let client_info = ClientInfo::generate();
    let sessions = vec![session::create(&rpc_client, &client_info).await?];

    Ok(Client {
        sessions,
        client_info,
        rpc_client,
    })
}

/// Create a client with user defined client info and sessions
/// TODO: maybe verify the session and return an error if session is invalid
pub fn new(client_info: ClientInfo, sessions: Vec<SessionEstablished>) -> Result<Client, Error> {
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
    pub sessions: Vec<SessionEstablished>,
    client_info: ClientInfo,
    rpc_client: rawclient::Client,
}

impl Client {
    /// Saves all your sessions to a file
    pub fn save_sessions(&self) -> Result<(), std::io::Error> {
        let filename = "social-void-rust.sessions";
        serde_json::to_writer(&std::fs::File::create(filename)?, &self.sessions)?;
        Ok(())
    }

    /// Loads all sessions from a file and adds them to the client
    pub fn load_sessions(&mut self, fpath: &str) -> Result<(), std::io::Error> {
        let sessions: Vec<SessionEstablished> =
            serde_json::from_reader(&std::fs::File::open(fpath)?)?;
        self.sessions.extend(sessions);
        Ok(())
    }

    /// Tries to establish another session adds it to the client if successful and returns the key of the session
    pub async fn new_session(&mut self) -> Result<usize, Error> {
        self.sessions
            .push(session::create(&self.rpc_client, &self.client_info).await?);

        Ok(self.sessions.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_get_a_session() {}
}