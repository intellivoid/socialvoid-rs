mod entities;

use entities::ClientInfo;
use entities::SessionEstablished;
use rawclient::Error;

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
}
