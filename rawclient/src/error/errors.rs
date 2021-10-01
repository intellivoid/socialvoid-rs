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

enum_from_primitive! {
    #[derive(Debug)]
    pub enum AuthenticationError {
        IncorrectLoginCredentials = 8704,
        IncorrectTwoFactorAuthenticationCode = 8705,
        AuthenticationNotApplicable = 8706,
        SessionNotFound = 8707,
        NotAuthenticated = 8708,
        PrivateAccessTokenRequired = 8709,
        AuthenticationFailure = 8710,
        BadSessionChallengeAnswer = 8711,
        TwoFactorAuthenticationRequired = 8712,
        AlreadyAuthenticated = 8713,
        SessionExpired = 8714,
    }
}

#[derive(Debug)]
pub enum ClientError {
    TermsOfServiceNotAgreed,
    SessionNotEstablished,
}
