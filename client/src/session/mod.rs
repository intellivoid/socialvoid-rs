mod entities;
mod session_challenge;

pub use entities::ClientInfo;
pub use entities::RegisterRequest;
pub use entities::Session;
pub use entities::SessionEstablished;
pub use entities::SessionHolder;
use entities::SessionRegisterInput;
use session_challenge::answer_challenge;
use socialvoid_rawclient::ClientError;
use socialvoid_rawclient::Error;
use socialvoid_types::Document;
pub use socialvoid_types::HelpDocument;
use socialvoid_types::Peer;
use socialvoid_types::SessionIdentification;

use serde_json::json;
use std::sync::Arc;
use std::sync::Mutex;

pub struct SVSessionMethods {
    client: Arc<socialvoid_rawclient::Client>,
    cdn_client: Arc<socialvoid_rawclient::CdnClient>,
    session: Arc<Mutex<SessionHolder>>,
}

impl SVSessionMethods {
    pub fn new(
        client: Arc<socialvoid_rawclient::Client>,
        cdn_client: Arc<socialvoid_rawclient::CdnClient>,
        session: Arc<Mutex<SessionHolder>>,
    ) -> Self {
        Self {
            client,
            cdn_client,
            session,
        }
    }

    /// `session.create`
    /// Creates a session and sets a session established object which contains a challenge.
    /// A session object is not yet returned - the challenge needs to be solved and sent inside a session identification
    /// object using the `get_session` method to get the Session object.
    pub async fn create(&self) -> Result<(), Error> {
        let mut session = self.session.lock().unwrap();
        let client_info = &*session.client_info.clone();
        session.established = Some(
            self.client
                .send_request("session.create", serde_json::value::to_value(client_info)?)
                .await?,
        );
        Ok(())
    }

    /// `session.get`
    /// Returns a `Session`
    pub async fn get(&self) -> Result<Session, Error> {
        let session_identification = self.session_identification()?;
        let mut session = self.session.lock().unwrap();
        let sesh: Session = self.client
            .send_request(
                "session.get",
                json!({"session_identification": serde_json::value::to_value(session_identification)?}),
            )
            .await?;
        session.authenticated = sesh.authenticated;
        Ok(sesh)
    }

    /// `session.authenticate_user`
    /// Authenticates a user via a username & password and optionally an OTP - extends session expiration time
    pub async fn authenticate_user(
        &self,
        username: String,
        password: String,
        otp: Option<String>,
    ) -> Result<bool, Error> {
        let session_identification = self.session_identification()?;
        let response = self
            .client
            .send_request(
                "session.authenticate_user",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "username": username,
                    "password": password,
                    "otp": otp
                }),
            )
            .await?;
        self.session.lock().unwrap().authenticated = true;
        Ok(response)
    }

    /// `session.logout`
    /// Log out without destroying the session - changes the session expiration date too
    pub async fn logout(&self) -> Result<bool, Error> {
        let session_identification = self.session_identification()?;
        let response = self
            .client
            .send_request(
                "session.logout",
                json!({
                    "session_identification":serde_json::value::to_value(session_identification)?
                }),
            )
            .await?;
        self.session.lock().unwrap().authenticated = false;
        Ok(response)
    }

    /// session.register
    /// Registers a new user to the network
    pub async fn register(&self, request: RegisterRequest) -> Result<Peer, Error> {
        let session_identification = self.session_identification()?;

        let request = SessionRegisterInput {
            session_identification,
            terms_of_service_id: self
                .session
                .lock()
                .unwrap()
                .tos_read
                .take()
                .ok_or_else(|| Error::new_client_error(ClientError::TermsOfServiceNotAgreed))?,
            terms_of_service_agree: true,
            username: request.username,
            password: request.password,
            first_name: request.first_name,
            last_name: request.last_name,
        };

        self.client
            .send_request("session.register", serde_json::to_value(request)?)
            .await
    }

    /// Upload a file to the CDN
    pub async fn upload_file(&self, file: &str) -> Result<Document, Error> {
        let session_identification = self.session_identification()?;
        self.cdn_client
            .upload(session_identification, file.to_string())
            .await
    }

    /// Download a file from the CDN
    pub async fn download_file(&self, document_id: String) -> Result<Vec<u8>, Error> {
        let session_identification = self.session_identification()?;
        self.cdn_client
            .download(session_identification, document_id)
            .await
    }

    /// Accepts the terms of service
    /// The client must explicitly call `session.accept_terms_of_service(terms_of_service)` to
    /// accept the terms of service. The HelpDocument can be acquired via `help::get_terms_of_service(socialvoid_rawclient)`
    pub fn accept_terms_of_service(&self, tos: HelpDocument) {
        self.session.lock().unwrap().tos_read = Some(tos.id);
    }

    pub fn session_identification(&self) -> Result<SessionIdentification, Error> {
        let session = self.session.lock().unwrap();
        if session.established.is_none() {
            return Err(Error::new_client_error(ClientError::SessionNotEstablished));
        }
        let session_id = session
            .established
            .as_ref()
            .map(|s| &s.id)
            .unwrap()
            .to_string();
        let challenge = session
            .established
            .as_ref()
            .map(|s| &s.challenge)
            .unwrap()
            .to_string();
        let client_public_hash = session.client_info.public_hash.clone();
        Ok(SessionIdentification {
            session_id,
            client_public_hash,
            challenge_answer: answer_challenge(session.client_info.private_hash.clone(), challenge),
        })
    }

    pub fn authenticated(&self) -> bool {
        self.session.lock().unwrap().authenticated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use entities::RegisterRequest;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use socialvoid_rawclient::{ClientError, Error, ErrorKind};
    #[tokio::test]
    async fn it_should_establish_a_session_and_get_it() -> Result<(), Error> {
        let session = SVSessionMethods::new(
            Arc::new(socialvoid_rawclient::new()),
            Arc::new(socialvoid_rawclient::CdnClient::new()),
            Arc::new(Mutex::new(SessionHolder::new(Arc::new(
                ClientInfo::generate(),
            )))),
        );
        session.create().await?;

        let sesh = session.get().await?;
        println!("{:?}", sesh);
        // assert_eq!(established.id, sesh.id);
        Ok(())
    }
    #[tokio::test]
    async fn it_should_establish_a_session_and_upload_and_download_a_file() -> Result<(), Error> {
        let creds: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string("../client/test_creds.test").unwrap())?;
        let session = SVSessionMethods::new(
            Arc::new(socialvoid_rawclient::new()),
            Arc::new(socialvoid_rawclient::CdnClient::new()),
            Arc::new(Mutex::new(SessionHolder::new(Arc::new(
                ClientInfo::generate(),
            )))),
        );
        session.create().await?;

        assert_eq!(
            session
                .authenticate_user(
                    creds["username"].as_str().unwrap().to_string(),
                    creds["password"].as_str().unwrap().to_string(),
                    None
                )
                .await?,
            true
        );

        let file_name = "test1.test";
        let file_content: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        std::fs::write(file_name, file_content.clone()).unwrap();

        let document = session.upload_file(file_name).await?;
        println!("Document: {:#?}", document);
        std::fs::remove_file(file_name).unwrap();

        let id = document.id;
        let downloaded_file = session.download_file(id).await?;
        assert_eq!(String::from_utf8_lossy(&downloaded_file), file_content);
        // assert_eq!(established.id, sesh.id);
        Ok(())
    }

    #[test]
    fn it_should_write_and_read_client_info_from_file() -> Result<(), std::io::Error> {
        let client_info_generated = ClientInfo::generate();
        let filename = "test_session_file.socialvoid.session";
        client_info_generated.save(filename)?;

        let client_info_read = ClientInfo::load_from_file(filename)?;

        assert_eq!(client_info_generated, client_info_read);

        Ok(())
    }

    #[tokio::test]
    async fn it_should_throw_a_terms_of_service_not_agreed_error() -> Result<(), Error> {
        let session = SVSessionMethods::new(
            Arc::new(socialvoid_rawclient::new()),
            Arc::new(socialvoid_rawclient::CdnClient::new()),
            Arc::new(Mutex::new(SessionHolder::new(Arc::new(
                ClientInfo::generate(),
            )))),
        );

        session.create().await?;
        let response = session
            .register(RegisterRequest {
                first_name: "Light".to_string(),
                last_name: None,
                username: "justanotherlight".to_string(),
                password: "SuperStrongPassword".to_string(),
            })
            .await;
        match response {
            Err(e) => match e.kind {
                ErrorKind::Client(e) => match e {
                    ClientError::TermsOfServiceNotAgreed => {}
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
        Ok(())
    }

    #[tokio::test]
    async fn it_should_calculate_the_correct_answer() {
        //This test creates a session and calculates the challenge answer
        // using the python script in the standard and the current implementation and
        // compares both
        use crate::ClientInfo;
        use std::process::Command;

        let client_info = ClientInfo::generate();
        let private_hash = client_info.private_hash.clone();
        let session = SVSessionMethods::new(
            Arc::new(socialvoid_rawclient::new()),
            Arc::new(socialvoid_rawclient::CdnClient::new()),
            Arc::new(Mutex::new(SessionHolder::new(Arc::new(
                ClientInfo::generate(),
            )))),
        );
        session.create().await.expect("Couldn't create the session");
        let established = session.session.lock().unwrap();
        let established = established.established.as_ref().unwrap();
        let challenge_answer =
            answer_challenge(private_hash.clone(), established.challenge.clone());

        let output = Command::new("python3")
            .arg("src/session/test-hotp.py")
            .arg(&private_hash)
            .arg(&established.challenge)
            .output()
            .expect("Couldn't run python script");
        println!(
            "output: {}, err: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&output.stdout), challenge_answer);
    }
}
