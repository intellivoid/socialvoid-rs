mod entities;

extern crate rawclient;

#[macro_use]
extern crate serde_json;

pub use entities::ClientInfo;
pub use entities::Session;
pub use entities::SessionEstablished;
pub use entities::SessionHolder;
pub use entities::SessionIdentification;

#[cfg(test)]
mod tests {
    use super::*;
    use rawclient::Error;
    #[tokio::test]
    async fn it_should_establish_a_session() -> Result<(), Error> {
        let session = SessionHolder::new(ClientInfo::generate());
        session.create(&rawclient::new()).await?;
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
}
