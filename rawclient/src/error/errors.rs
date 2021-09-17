enum_from_primitive! {
    #[derive(Debug)]
    pub enum RpcError {
        ParseError = -32700,
        InvalidRequest = -32600,
        MethodNotFound = -32601,
        InvalidParams = -32602,
        InternalError = -32603,
    }
}

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
