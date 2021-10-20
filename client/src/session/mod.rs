mod entities;

pub use entities::ClientInfo;
pub use entities::RegisterRequest;
pub use entities::Session;
pub use entities::SessionEstablished;
pub use entities::SessionHolder;

#[cfg(test)]
mod tests {
    use super::*;
    use entities::RegisterRequest;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use socialvoid_rawclient::{ClientError, Error, ErrorKind};
    #[tokio::test]
    async fn it_should_establish_a_session_and_get_it() -> Result<(), Error> {
        let mut session = SessionHolder::new(ClientInfo::generate());
        let client = socialvoid_rawclient::new();
        session.create(&client).await?;

        let sesh = session.get(&client).await?;
        println!("{:?}", sesh);
        // assert_eq!(established.id, sesh.id);
        Ok(())
    }
    #[tokio::test]
    async fn it_should_establish_a_session_and_upload_and_download_a_file() -> Result<(), Error> {
        let creds: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string("../client/test_creds.test").unwrap())?;
        let mut session = SessionHolder::new(ClientInfo::generate());
        let client = socialvoid_rawclient::new();
        session.create(&client).await?;

        assert_eq!(
            session
                .authenticate_user(
                    &client,
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
        let cdn_client = socialvoid_rawclient::CdnClient::new();
        let document = session.upload_file(file_name, &cdn_client).await?;
        println!("Document: {:#?}", document);
        std::fs::remove_file(file_name).unwrap();

        let id = document.id;
        let downloaded_file = session.download_file(id, &cdn_client).await?;
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
        let mut session = SessionHolder::new(ClientInfo::generate());
        let client = socialvoid_rawclient::new();
        session.create(&client).await?;
        let response = session
            .register(
                RegisterRequest {
                    first_name: "Light".to_string(),
                    last_name: None,
                    username: "justanotherlight".to_string(),
                    password: "SuperStrongPassword".to_string(),
                },
                &client,
            )
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
}