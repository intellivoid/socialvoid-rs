use crate::SVSessionMethods;
use serde_json::json;
use socialvoid_rawclient as rawclient;
use socialvoid_rawclient::Error;
use socialvoid_types::Peer;
use socialvoid_types::Profile;
use socialvoid_types::RelationshipType;
use std::sync::Arc;

pub struct SVNetworkMethods {
    client: Arc<rawclient::Client>,
    session: Arc<SVSessionMethods>,
}

impl SVNetworkMethods {
    pub fn new(client: Arc<rawclient::Client>, session: Arc<SVSessionMethods>) -> SVNetworkMethods {
        SVNetworkMethods { client, session }
    }

    /// GetMe
    /// Returns the peer object of the authenticated peer
    pub async fn get_me(&self) -> Result<Peer, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
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
    pub async fn get_profile(&self, peer: Option<String>) -> Result<Profile, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
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
    pub async fn resolve_peer(&self, peer: String) -> Result<Peer, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
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
    pub async fn unfollow_peer(&self, peer: String) -> Result<RelationshipType, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "network.unfollow_peer",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "peer": peer,
                }),
            )
            .await
    }

    /// FollowPeer
    pub async fn follow_peer(&self, peer: String) -> Result<RelationshipType, Error> {
        let session_identification = self.session.session_identification()?;
        self.client
            .send_request(
                "network.follow_peer",
                json!({
                    "session_identification": serde_json::to_value(session_identification)?,
                    "peer": peer,
                }),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{ClientInfo, SVSessionMethods, SessionHolder};
    use rawclient::{AuthenticationError, ErrorKind};
    use std::sync::{Arc, Mutex};
    #[tokio::test]
    async fn it_should_return_a_notauthenticated_error() {
        let client = Arc::new(socialvoid_rawclient::new());
        let session = Arc::new(SVSessionMethods::new(
            Arc::clone(&client),
            Arc::new(socialvoid_rawclient::CdnClient::new()),
            Arc::new(Mutex::new(SessionHolder::new(Arc::new(
                ClientInfo::generate(),
            )))),
        ));
        session.create().await.expect("Couldn't create a session.");

        let network = SVNetworkMethods::new(Arc::clone(&client), Arc::clone(&session));
        match network.get_me().await {
            Ok(_) => panic!("Session found for some reason.?"),
            Err(error) => match error.kind {
                ErrorKind::Authentication(error) => match error {
                    AuthenticationError::NotAuthenticated => {}
                    authkind => panic!("Unexpected authentication error: {:#?}", authkind),
                },
                kind => panic!("Unexpected error: {:#?}", kind),
            },
        }
    }
}
