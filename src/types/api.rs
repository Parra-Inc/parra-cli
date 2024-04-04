use serde::{Deserialize, Serialize};

use super::auth::Credental;

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoResponse {
    pub user: UserResponse,
}

#[derive(Debug)]
pub struct AuthorizedUser {
    pub credential: Credental,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct TenantRequest {
    pub name: String,
    pub is_test: bool,
}

#[derive(Debug, Deserialize)]
pub struct TenantResponse {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ApplicationType {
    #[serde(rename = "ios")]
    Ios,
}

#[derive(Debug, Serialize)]
pub struct ApplicationRequest {
    pub name: String,
    pub description: Option<String>,
    pub r#type: ApplicationType,
    /// This is currently only required when `type` is `ios`. But since that's the only type
    /// we support right now, we're making it required here.
    pub ios_bundle_id: String,
    pub is_new_project: bool,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationIosConfig {
    pub bundle_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub r#type: ApplicationType,
    pub tenant_id: String,
    /// Will always be present when `type` is `ios`.
    pub ios: Option<ApplicationIosConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationCollectionResponse {
    pub data: Vec<ApplicationResponse>,
}
