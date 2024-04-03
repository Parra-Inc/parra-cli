use std::error::Error;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{auth, types::Credental};

#[derive(Debug, Deserialize)]
pub struct Tenant {}

pub async fn getTenants() -> Result<Vec<Tenant>, Box<dyn Error>> {
    let credential = ensure_auth().await?;

    // let tenants: Vec<Tenant> =
    //     get_request("https://api.parra.io/tenants").await?;

    // for tenant in tenants {
    //     println!("Tenant: {}", tenant.name);
    // }

    Ok(vec![])
}

async fn ensure_auth() -> Result<Credental, Box<dyn Error>> {
    return auth::perform_device_authentication().await;
}

async fn perform_request<T: DeserializeOwned, U: Serialize>(
    credential: &Credental,
    endpoint: &str,
    method: reqwest::Method,
    body: Option<U>,
) -> Result<T, Box<dyn Error>> {
    let url = format!("https://api.parra.io/v1{}", endpoint);
    let client = reqwest::Client::new();
    let mut request = client.request(method, url).bearer_auth(credential.token);

    if let Some(body) = body {
        if method != reqwest::Method::GET {
            request = request.json(&body);
        }
    }

    let response = request.send().await?;

    if response.status().is_success() {
        Ok(response.text()?)
    } else {
        Err("Request failed".into())
    }
}
