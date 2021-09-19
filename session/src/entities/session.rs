use rawclient::Error;
use types::Peer;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionIdentification {
    session_id: String,
    client_public_hash: String,
    challenge_answer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    id: String,
    flags: Vec<String>,
    authenticated: bool,
    created: i32,
    expires: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionEstablished {
    pub id: String,
    pub challenge: String,
}

impl SessionEstablished {
    /// `session.get`
    /// Returns a `Session`
    pub async fn get(&self, rpc_client: &rawclient::Client) -> Result<Session, Error> {
        let session_identification = self.session_identification();
        rpc_client
            .send_request(
                "session.get",
                serde_json::value::to_value(session_identification)?,
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
        let session_identification = self.session_identification();
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
        let session_identification = self.session_identification();
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
        let session_identification = self.session_identification();
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

    //TODO: solve the challenge
    fn session_identification(&self) -> SessionIdentification {
        SessionIdentification {
            session_id: "dummy".to_string(),
            client_public_hash: "dummy".to_string(),
            challenge_answer: "dummy".to_string(),
        }
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
