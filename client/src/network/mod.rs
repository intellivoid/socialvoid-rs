use serde_json::json;
use socialvoid_rawclient as rawclient;
use socialvoid_rawclient::Error;
use socialvoid_types::Peer;
use socialvoid_types::Profile;
use socialvoid_types::RelationshipType;
use socialvoid_types::SessionIdentification;

/// GetMe
/// Returns the peer object of the authenticated peer
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

/// GetProfile
/// `peer` can be 'None' for own profile, otherwise,
/// 'peer' can be Some(p) where p can be the id or username(with leading @) of the peer.
pub async fn get_profile(
    client: &rawclient::Client,
    session_identification: SessionIdentification,
    peer: Option<String>,
) -> Result<Profile, Error> {
    client
        .send_request(
            "network.get_profile",
            json!({
                "session_identification": serde_json::to_value(session_identification)?,
                "peer": peer,
            }),
        )
        .await
}

/// ResolvePeer
pub async fn resolve_peer(
    client: &rawclient::Client,
    session_identification: SessionIdentification,
    peer: String,
) -> Result<Peer, Error> {
    client
        .send_request(
            "network.resolve_peer",
            json!({
                "session_identification": serde_json::to_value(session_identification)?,
                "peer": peer,
            }),
        )
        .await
}

/// UnfollowPeer
pub async fn unfollow_peer(
    client: &rawclient::Client,
    session_identification: SessionIdentification,
    peer: String,
) -> Result<RelationshipType, Error> {
    client
        .send_request(
            "network.unfollow_peer",
            json!({
                "session_identification": serde_json::to_value(session_identification)?,
                "peer": peer,
            }),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{ClientInfo, SessionHolder};
    use rawclient::{AuthenticationError, ErrorKind};
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
