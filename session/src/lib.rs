mod entities;

use entities::ClientInfo;
use entities::SessionEstablished;
use rawclient::Error;

/// Creates a session and returns a session established object which contains a challenge
pub async fn create(client: &rawclient::Client) -> Result<SessionEstablished, Error> {
    let client_info = ClientInfo::generate();
    client
        .send_request("session.create", serde_json::value::to_value(client_info)?)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn it_should_establish_a_session() {
        let client = rawclient::new();
        let response = create(&client).await;
        match response {
            Ok(s_e) => {
                println!("Result: {:?}", s_e);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                unreachable!()
            }
        }
    }

    #[test]
    fn it_should_write_and_read_client_info_from_file() -> Result<(), std::io::Error> {
        let client_info_generated = ClientInfo::generate();
        let filename = "test_session_file.socialvoid.session";
        client_info_generated.save(filename)?;

        let client_info_read = ClientInfo::load_from_file(filename)?;
        println!("{:?}\n", client_info_read);

        assert_eq!(client_info_generated, client_info_read);

        Ok(())
    }
}
