use rawclient::Error;
use serde_json::json;
use types::Peer;
use types::SessionIdentification;

pub async fn get_me(
    client: &rawclient::Client,
    session_identification: SessionIdentification,
) -> Result<Peer, Error> {
    client
        .send_request(
            "network.get_me",
            json!({
                "session_identification": serde_json::to_value(session_identification)?
            }),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use rawclient::{AuthenticationError, ErrorKind};
    use session::{ClientInfo, SessionHolder};
    #[tokio::test]
    async fn it_should_return_a_session_not_found_error_if_session_unauthenticated() {
        let client = rawclient::new();
        let mut session = SessionHolder::new(ClientInfo::generate());
        session
            .create(&client)
            .await
            .expect("Couldn't create a session.");
        match get_me(
            &client,
            session
                .session_identification()
                .expect("Couldn't get session identification object (unestablished session)"),
        )
        .await
        {
            Ok(_) => unreachable!(),
            Err(error) => match error.kind {
                ErrorKind::Authentication(error) => match error {
                    AuthenticationError::SessionNotFound => {}
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
        }
    }
}
