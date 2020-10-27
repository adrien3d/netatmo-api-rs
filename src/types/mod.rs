pub mod home;
pub mod station;

use serde::{Deserialize};

#[derive(Clone, Default, Deserialize)]
pub struct AuthApiResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub scope: Vec<String>,
    //expires_in: u64,
    pub expire_in: u64,
}

#[derive(Clone, Default, Deserialize)]
pub struct AuthApiErrorResponse {
    pub error: String,
    pub error_description: String
}
