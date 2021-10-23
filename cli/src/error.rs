use socialvoid::{ClientError, SocialvoidError};
use socialvoid_rawclient::ErrorKind;

pub struct MyFriendlyError(SocialvoidError);

impl std::convert::From<SocialvoidError> for MyFriendlyError {
    fn from(err: SocialvoidError) -> Self {
        Self(err.into())
    }
}

impl std::fmt::Display for MyFriendlyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            SocialvoidError::RawClient(err) => match &err.kind {
                ErrorKind::Authentication(err) => {
                    write!(f, "This method needs you to log in.
Authentication Error: {:#?}\nIf you are already logged in, then try logging out and logging in again.
To log in:
socialvoid-cli login
To log out:
socialvoid-cli logout", err)
                }
                ErrorKind::Cdn(err) => {
                    write!(
                        f,
                        "There was a problem while uploading/downloading the file from CDN.
CDN Error: {:#?}\nIf it was an authentication error, try logging in.
To log in:
socialvoid-cli login
To log out:
socialvoid-cli logout",
                        err
                    )
                }
                _ => {
                    write!(f, "{:#?}", self.0)
                }
            },
            SocialvoidError::Client(err) => match err {
                ClientError::NoSessionsExist => {
                    write!(f, "Seems you need to create a session.")
                }
                ClientError::SerdeJson(err) => {
                    write!(f, "Error while parsing JSON.\n{:?}", err)
                }
            },
        }
    }
}
