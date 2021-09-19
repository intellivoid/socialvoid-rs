use rawclient::Error;

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
        let session_identification = &self.id;
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
        let session_identification = &self.id;
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
        let session_identification = &self.id;
        rpc_client
            .send_request(
                "session.logout",
                serde_json::value::to_value(session_identification)?,
            )
            .await
    }
}
