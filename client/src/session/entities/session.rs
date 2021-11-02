use super::ClientInfo;
use serde::{Deserialize, Serialize};
use socialvoid_types::SessionIdentification;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub flags: Vec<String>,
    pub authenticated: bool,
    pub created: i32,
    pub expires: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionEstablished {
    pub id: String,
    pub challenge: String,
}

//TODO: maybe do serde thing
#[derive(Debug)]
pub struct SessionHolder {
    pub established: Option<SessionEstablished>,
    pub authenticated: bool,
    pub client_info: Arc<ClientInfo>,
    pub tos_read: Option<String>, //Holds the terms of service ID
}

impl SessionHolder {
    pub fn new(client_info: Arc<ClientInfo>) -> SessionHolder {
        SessionHolder {
            established: None,
            client_info,
            tos_read: None,
            authenticated: false,
        }
    }
}

pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SessionRegisterInput {
    pub session_identification: SessionIdentification,
    pub terms_of_service_id: String,
    pub terms_of_service_agree: bool,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
}
