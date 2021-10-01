use super::ClientInfo;
use rawclient::ClientError;
use rawclient::Error;
use serde::{Deserialize, Serialize};
use types::Peer;

use super::session_challenge::answer_challenge;

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionIdentification {
    session_id: String,
    client_public_hash: String,
    challenge_answer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub flags: Vec<String>,
    pub authenticated: bool,
    pub created: i32,
    pub expires: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionEstablished {
    pub id: String,
    pub challenge: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionHolder {
    pub established: Option<SessionEstablished>,
    client_info: ClientInfo,
}

impl SessionHolder {
    pub fn new(client_info: ClientInfo) -> SessionHolder {
        SessionHolder {
            established: None,
            client_info,
        }
    }

    /// `session.create`
    /// Creates a session and sets a session established object which contains a challenge.
    /// A session object is not yet returned - the challenge needs to be solved and sent inside a session identification
    /// object using the `get_session` method to get the Session object.
    pub async fn create(&mut self, rpc_client: &rawclient::Client) -> Result<(), Error> {
        let client_info = &self.client_info;
        self.established = Some(
            rpc_client
                .send_request("session.create", serde_json::value::to_value(client_info)?)
                .await?,
        );
        Ok(())
    }

    /// `session.get`
    /// Returns a `Session`
    pub async fn get(&self, rpc_client: &rawclient::Client) -> Result<Session, Error> {
        let session_identification = self.session_identification()?;
        rpc_client
            .send_request(
                "session.get",
                json!({"session_identification": serde_json::value::to_value(session_identification)?}),
            )
            .await
    }

    /// `session.authenticate_user`
    /// Authenticates a user via a username & password and optionally an OTP - extends session expiration time
    pub async fn authenticate_user(
        &self,
        rpc_client: &rawclient::Client,
        username: String,
        password: String,
        otp: Option<String>,
    ) -> Result<bool, Error> {
        let session_identification = self.session_identification()?;
        rpc_client
            .send_request(
                "session.authenticate_user",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "username": username,
                    "password": password,
                    "otp": otp
                }),
            )
            .await
    }

    /// `session.logout`
    /// Log out without destroying the session - changes the session expiration date too
    pub async fn logout(&self, rpc_client: &rawclient::Client) -> Result<bool, Error> {
        let session_identification = self.session_identification()?;
        rpc_client
            .send_request(
                "session.logout",
                serde_json::value::to_value(session_identification)?,
            )
            .await
    }

    /// session.register
    /// Registers a new user to the network
    pub async fn register(
        &self,
        request: RegisterRequest,
        rpc_client: &rawclient::Client,
    ) -> Result<Peer, Error> {
        let session_identification = self.session_identification()?;
        let request = SessionRegisterInput {
            session_identification,
            terms_of_service_id: request.terms_of_service_id,
            terms_of_service_agree: request.terms_of_service_agree,
            username: request.username,
            password: request.password,
            first_name: request.first_name,
            last_name: request.last_name,
        };

        rpc_client
            .send_request("session.register", serde_json::to_value(request)?)
            .await
    }

    pub fn session_identification(&self) -> Result<SessionIdentification, Error> {
        if self.established.is_none() {
            return Err(Error::new_client_error(ClientError::SessionNotEstablished));
        }
        let session_id = self
            .established
            .as_ref()
            .map(|s| &s.id)
            .unwrap()
            .to_string();
        let challenge = self
            .established
            .as_ref()
            .map(|s| &s.challenge)
            .unwrap()
            .to_string();
        let client_public_hash = self.client_info.public_hash.clone();
        Ok(SessionIdentification {
            session_id,
            client_public_hash,
            challenge_answer: answer_challenge(self.client_info.private_hash.clone(), challenge),
        })
    }
}

#[derive(Serialize, Debug)]
struct SessionRegisterInput {
    session_identification: SessionIdentification,
    terms_of_service_id: String,
    terms_of_service_agree: bool,
    username: String,
    password: String,
    first_name: String,
    last_name: Option<String>,
}

pub struct RegisterRequest {
    pub terms_of_service_agree: bool,
    pub terms_of_service_id: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
}
