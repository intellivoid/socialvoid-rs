pub mod account;
pub mod error;
pub mod help;
pub mod network;
pub mod session;
pub mod timeline;

use account::SVAccountMethods;
pub use error::ClientError;
pub use error::SocialvoidError;
use help::SVHelpMethods;
use network::SVNetworkMethods;
use session::ClientInfo;
use session::SVSessionMethods;
use session::SessionHolder;
use std::sync::{Arc, Mutex};
use timeline::SVTimelineMethods;

/// A client that can be used to call methods and manage sessions for Social Void
pub struct Client {
    cdn_client: Arc<socialvoid_rawclient::CdnClient>,
    pub help: Arc<SVHelpMethods>,
    pub session: Arc<SVSessionMethods>,
    pub network: Arc<SVNetworkMethods>,
    pub account: Arc<SVAccountMethods>,
    pub timeline: Arc<SVTimelineMethods>,
}

/// Create a client and establish a new session
pub async fn new_with_defaults() -> Result<Client, SocialvoidError> {
    let rpc_client = Arc::new(socialvoid_rawclient::new());
    let cdn_client = Arc::new(make_cdn_client_from(Arc::clone(&rpc_client)).await?);
    let client_info = Arc::new(ClientInfo::generate());
    let session_holder = Arc::new(Mutex::new(SessionHolder::new(Arc::clone(&client_info))));
    let (session, network, account, timeline, help) = init_methods(
        Arc::clone(&rpc_client),
        Arc::clone(&cdn_client),
        Arc::clone(&session_holder),
    );

    session.create().await?;
    let client = Client {
        cdn_client,
        help,
        network,
        session,
        account,
        timeline,
    };
    Ok(client)
}

/// Creates the CDN client by resolving the host url from server information
async fn make_cdn_client_from(
    rpc_client: Arc<socialvoid_rawclient::Client>,
) -> Result<socialvoid_rawclient::CdnClient, SocialvoidError> {
    let server_info = SVHelpMethods::new(Arc::clone(&rpc_client))
        .get_server_information()
        .await?;

    Ok(socialvoid_rawclient::CdnClient::with_cdn_url(
        server_info.cdn_server,
    ))
}

/// Creates a new client with the given rpc url, default cdn url and no sessions
pub fn new_with_host(rpc_url: Option<String>) -> Client {
    let rpc_client = if let Some(rpc_url) = rpc_url {
        Arc::new(socialvoid_rawclient::with_host(&rpc_url))
    } else {
        Arc::new(socialvoid_rawclient::new())
    };
    let cdn_client = Arc::new(socialvoid_rawclient::CdnClient::new());
    let client_info = Arc::new(ClientInfo::generate());
    let session_holder = Arc::new(Mutex::new(SessionHolder::new(Arc::clone(&client_info))));
    let (session, network, account, timeline, help) = init_methods(
        Arc::clone(&rpc_client),
        Arc::clone(&cdn_client),
        Arc::clone(&session_holder),
    );
    Client {
        session,
        cdn_client,
        help,
        timeline,
        network,
        account,
    }
}

/// Create a client with user defined session, (optional)rpc server url and (optional)cdn server url
/// And CDN as given in the server information
/// TODO: maybe verify the session and return an error if session is invalid
pub async fn new(
    session: SessionHolder,
    rpc_url: Option<String>,
    cdn_url: Option<String>,
) -> Result<Client, SocialvoidError> {
    let rpc_client = if let Some(rpc_url) = rpc_url {
        Arc::new(socialvoid_rawclient::with_host(&rpc_url))
    } else {
        Arc::new(socialvoid_rawclient::new())
    };
    let cdn_client = if let Some(cdn_url) = cdn_url {
        Arc::new(socialvoid_rawclient::CdnClient::with_cdn_url(cdn_url))
    } else {
        Arc::new(make_cdn_client_from(Arc::clone(&rpc_client)).await?)
    };
    let session_holder = Arc::new(Mutex::new(session));
    let (session, network, account, timeline, help) = init_methods(
        Arc::clone(&rpc_client),
        Arc::clone(&cdn_client),
        Arc::clone(&session_holder),
    );
    Ok(Client {
        session,
        cdn_client,
        help,
        timeline,
        network,
        account,
    })
}

/// Create a client with generated client info and zero sessions
/// Note that, cdn client may not be the one taken from server information
pub fn new_empty_client() -> Client {
    let rpc_client = Arc::new(socialvoid_rawclient::new());
    let client_info = Arc::new(ClientInfo::generate());
    let session_holder = Arc::new(Mutex::new(SessionHolder::new(Arc::clone(&client_info))));
    let (session, network, account, timeline, help) = init_methods(
        Arc::clone(&rpc_client),
        Arc::new(socialvoid_rawclient::CdnClient::new()),
        Arc::clone(&session_holder),
    );
    Client {
        cdn_client: Arc::new(socialvoid_rawclient::CdnClient::new()),
        help,
        timeline,
        network,
        session,
        account,
    }
}

pub fn init_methods(
    client: Arc<socialvoid_rawclient::Client>,
    cdn_client: Arc<socialvoid_rawclient::CdnClient>,
    session_holder: Arc<Mutex<SessionHolder>>,
) -> (
    Arc<SVSessionMethods>,
    Arc<SVNetworkMethods>,
    Arc<SVAccountMethods>,
    Arc<SVTimelineMethods>,
    Arc<SVHelpMethods>,
) {
    let session = Arc::new(SVSessionMethods::new(
        Arc::clone(&client),
        Arc::clone(&cdn_client),
        Arc::clone(&session_holder),
    ));
    (
        Arc::clone(&session),
        Arc::new(SVNetworkMethods::new(
            Arc::clone(&client),
            Arc::clone(&session),
        )),
        Arc::new(SVAccountMethods::new(
            Arc::clone(&client),
            Arc::clone(&session),
        )),
        Arc::new(SVTimelineMethods::new(
            Arc::clone(&client),
            Arc::clone(&session),
        )),
        Arc::new(SVHelpMethods::new(Arc::clone(&client))),
    )
}

impl Client {
    /// Set the CDN server URL from the ServerInfomation
    pub async fn reset_cdn_url(&mut self) -> Result<(), SocialvoidError> {
        self.cdn_client =
            Arc::new(make_cdn_client_from(Arc::new(socialvoid_rawclient::new())).await?); //todo: maybe propagate the change??
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    const CREDS_FILE_1: &str = "test_creds.test";

    #[tokio::test]
    async fn it_should_log_in_and_get_the_correct_peer() -> Result<(), SocialvoidError> {
        // let sessions_file = "sessions.test";

        let creds: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(CREDS_FILE_1).unwrap())?;

        let sv = new_with_defaults().await?;
        sv.session
            .authenticate_user(
                creds["username"].as_str().unwrap().to_string(),
                creds["password"].as_str().unwrap().to_string(),
                None,
            )
            .await?;

        let peer = sv.network.get_me().await?;

        println!("{:?}", peer);
        sv.session.logout().await?;
        assert_eq!(
            peer.username,
            creds["username"].as_str().unwrap().to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn it_should_create_post_and_delete_it() -> Result<(), SocialvoidError> {
        let creds: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(CREDS_FILE_1).unwrap())?;
        let sv = new_with_defaults().await?;
        sv.session
            .authenticate_user(
                creds["username"].as_str().unwrap().to_string(),
                creds["password"].as_str().unwrap().to_string(),
                None,
            )
            .await?;

        let post_text = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect::<String>();
        let post = sv.timeline.compose(&post_text, Vec::new()).await?;
        if !sv.timeline.delete(post.id).await? {
            panic!("Delete post returned false unexpectedly.")
        }
        Ok(())
    }
}
