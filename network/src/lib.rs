use rawclient::{AuthenticationError, Error, ErrorKind};
use session::SessionIdentification;
use types::Peer;

pub async fn get_me(
    _client: &rawclient::Client,
    _session_identification: SessionIdentification,
) -> Result<Peer, Error> {
    Err(Error {
        code: 8707,
        kind: ErrorKind::Authentication(AuthenticationError::SessionNotFound),
        description: "Raised when the requested session was not found in the network".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
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
