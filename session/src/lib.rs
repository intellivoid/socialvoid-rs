mod entities;

extern crate rawclient;

#[macro_use]
extern crate serde_json;

use rawclient::Error;

pub use entities::ClientInfo;
pub use entities::Session;
pub use entities::SessionEstablished;
pub use entities::SessionHolder;
pub use entities::SessionIdentification;

/// `session.create`
/// Creates a session and returns a session established object which contains a challenge.
/// A session object is not yet returned - the challenge needs to be solved and sent inside a session identification
/// object using the `get_session` method to get the Session object.
pub async fn create(
    rpc_client: &rawclient::Client,
    client_info: &ClientInfo,
) -> Result<SessionEstablished, Error> {
    rpc_client
        .send_request("session.create", serde_json::value::to_value(client_info)?)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn it_should_establish_a_session() -> Result<(), Error> {
        create(&rawclient::new(), &ClientInfo::generate()).await?;
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
