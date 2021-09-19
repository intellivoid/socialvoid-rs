use super::ClientInfo;
use rawclient::Error;
use types::Peer;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1;
use pad::{Alignment, PadStr};
use serde::{Deserialize, Serialize};

use std::io::Cursor;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub struct SessionHolder {
    established: Option<SessionEstablished>,
    client_info: ClientInfo,
}

impl SessionHolder {
    /// `session.get`
    /// Returns a `Session`
    pub async fn get(&self, rpc_client: &rawclient::Client) -> Result<Session, Error> {
        let session_identification = self.session_identification()?;
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

    //TODO: solve the challenge
    fn session_identification(&self) -> Result<SessionIdentification, String> {
        if self.established.is_none() {
            return Err(String::from("Session not established."));
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

/// Returns the challenge_answer using the SessionEstablished object
pub fn answer_challenge(client_private_hash: String, challenge: String) -> String {
    let mut hasher = sha1::Sha1::new();
    let totp_code = totp(challenge.to_string());
    //hashlib.sha1("{0}{1}".format(totp_code, client_private_hash).encode()).hexdigest()
    hasher.input(&format!("{}{}", totp_code, client_private_hash).as_bytes());
    hasher.result_str()
}

fn totp(key: String) -> String {
    let time_step = 30;
    let now = SystemTime::now();
    let counter = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        / time_step;
    let digits = 6;
    hotp(key, counter, digits)
}
fn hotp(key: String, counter: u64, digits: u8) -> String {
    let key = base32::decode(
        base32::Alphabet::RFC4648 { padding: true },
        &format!("{}{}", key.to_uppercase(), "=".repeat((8 - key.len()) % 8)),
    )
    .expect("Couldn't decode base32");
    let mut hmac = Hmac::new(sha1::Sha1::new(), &key);
    let mut msg = vec![];
    msg.write_u64::<BigEndian>(counter).unwrap();
    hmac.input(&msg);
    let mut result = vec![];
    hmac.raw_result(&mut result);
    let offset = result.last().unwrap() & 0x0f;
    let mut rdr = Cursor::new(&result[offset as usize..(offset + 4) as usize]);
    let binary = rdr.read_u32::<BigEndian>().unwrap() & 0x7fffffff;
    let otp = binary
        .to_string()
        .pad(digits as usize, '0', Alignment::Right, true);
    otp
}
