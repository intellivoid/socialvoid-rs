use socialvoid_rawclient::ErrorKind;

pub struct MyFriendlyError(socialvoid_rawclient::Error);

impl std::convert::From<socialvoid_rawclient::Error> for MyFriendlyError {
    fn from(err: socialvoid_rawclient::Error) -> Self {
        Self(err)
    }
}

impl std::fmt::Display for MyFriendlyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0.kind {
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
        }
    }
}
