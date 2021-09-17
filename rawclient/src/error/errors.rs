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
        InvalidFirstName = 8450,
        InvalidLastName = 8451,
        InvalidBiography = 8452,
        UsernameAlreadyExists = 8453,
        InvalidPeerInput = 8454,
        InvalidPostText = 8455,
        InvalidClientPublicHash = 8456,
        InvalidClientPrivateHash = 8457,
        InvalidPlatform = 8458,
        InvalidVersion = 8459,
        InvalidClientName = 8460,
        InvalidSessionIdentification = 8461,
        InvalidFileForProfilePicture = 8462,
        FileTooLarge = 8463,
        InvalidHelpDocumentId = 8464,
        AgreementRequired = 8465,
    }
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum ServerError {
        InternalServerError = 16384,
        DocumentUpload = 16385
    }
}
