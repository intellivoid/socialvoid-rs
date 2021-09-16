enum_from_primitive! {
#[derive(Debug)]
pub enum ValidationError {
    InvalidUsername = 8448,
    InvalidPassword = 8449,
}
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum ServerError {
        InternalServerError = 16384,
        DocumentUpload = 16385
    }
}
