use serde::{Deserialize, Serialize};
use std::fmt;

// #[derive(Debug)]
// pub struct AuthResponse {
//     pub application_id: String,
//     pub application_name: String,
//     pub tenant_id: String,
// }

#[derive(Debug)]
pub struct AuthInfo {
    pub application_id: String,
    pub tenant_id: String,
}

#[derive(Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub path: String,
}

#[derive(Copy, Clone)]
pub struct XcodeVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl fmt::Debug for XcodeVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Xcode version: {}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

// Auth

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Credental {
    pub token: String,
    pub expiry: u64,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DeviceAuthResponse {
    /// Do not show this to the user. It is used to poll for the token.
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    /// lifetime in seconds for device_code and user_code. Default 900.
    pub expires_in: u64,
    /// The interval to poll the token endpoint. Default 5.
    pub interval: u64,
}

#[derive(Debug, Serialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub device_code: String,
    pub client_id: String,
}
