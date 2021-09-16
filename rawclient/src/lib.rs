/// # Raw client for SocialVoid.
/// Makes a new client and makes JSONRPC requests. Also, useful in case we
/// want to switch the JSONRPC client crate used in the future.
mod error;

pub use error::Error;

const HOST: &str = "http://socialvoid.qlg1.com:5601/";

pub struct Client {
    client: jsonrpc2_client::Client,
}

pub fn new() -> Client {
    let host = get_host();
    Client {
        client: jsonrpc2_client::new(&host),
    }
}

impl Client {
    pub async fn send_request<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, Error> {
        let response = self.client.send_request::<T>(method, params).await?;
        Ok(response)
    }
}

fn get_host() -> String {
    HOST.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
