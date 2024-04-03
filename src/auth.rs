use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize)]
struct AuthRequest {
    client_id: String,
    redirect_uri: String,
    response_type: String,
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

const AUTH0_CLIENT_ID: &str = "nD9GTUvvqCT0oWi34L2IdJiK0YjupSjY";

// pub fn perform_authentication() {
//     let callback_path = "callback".to_string();
//     let callback_uri = format!("http://localhost:7272/{}", callback_path);

//     let auth_request = AuthRequest {
//         client_id: "nD9GTUvvqCT0oWi34L2IdJiK0YjupSjY".to_string(),
//         redirect_uri: callback_uri,
//         response_type: "token".to_string(),
//     };

//     let auth_url = "https://parra.auth0.com/authorize?".to_string()
//         + &form_urlencoded::Serializer::new(String::new())
//             .append_pair("client_id", &auth_request.client_id)
//             .append_pair("redirect_uri", &auth_request.redirect_uri)
//             .append_pair("response_type", &auth_request.response_type)
//             .append_pair("response_mode", "form_post")
//             .finish();

//     let auth_response: AuthResponse =
//         open_url_and_wait_for_callback(&auth_url, callback_path);

//     println!("Auth response: {:?}", auth_response);
// }

pub async fn perform_device_authentication() -> Result<(), Box<dyn Error>> {
    println!("Performing device authentication with Parra API.");

    let device_code_url = "https://parra.auth0.com/oauth/device/code";

    let device_auth_response: Result<DeviceAuthResponse, Box<dyn Error>> =
        post_form_request(
            device_code_url,
            vec![
                ("client_id".to_string(), AUTH0_CLIENT_ID.to_string()),
                // ("scope".to_string(), "openid profile email".to_string()),
            ],
        )
        .await;

    println!("Device auth response: {:?}", device_auth_response);

    let device_auth = device_auth_response.unwrap();
    let result = open::that(device_auth.verification_uri_complete);

    if result.is_err() {
        println!(
            "Failed to open the browser. Please visit {} and enter the code: {} to confirm your login.",
            device_auth.verification_uri,
            device_auth.user_code
        );
    }

    // begin polling for the token
    let token_url = "https://parra.auth0.com/oauth/token";

    let token_request_body = TokenRequest {
        client_id: AUTH0_CLIENT_ID.to_string(),
        device_code: device_auth.device_code,
        grant_type: "urn:ietf:params:oauth:grant-type:device_code".to_string(),
    };

    // Strictly speaking, the polling should begin before the user is prompted. But since
    // we don't have a UI that requires user interaction to launch the browser, and we
    // aren't awaiting the result of that web page being opened, we can launch it, then
    // begin polling.
    let poll_result = poll_for_token::<AuthResponse>(
        token_url,
        device_auth.interval,
        device_auth.expires_in,
        token_request_body,
    )
    .await?;

    persist_credentials_struct(&poll_result)?;

    println!("Authentication successful!");

    Ok(())
}

fn persist_credentials_struct(
    data: &AuthResponse,
) -> Result<(), Box<dyn Error>> {
    let serialized = serde_json::to_string(data).unwrap();

    let existing = security_framework::passwords::get_generic_password(
        "parra_cli",
        AUTH0_CLIENT_ID,
    );

    security_framework::passwords::set_generic_password(
        "parra_cli",
        AUTH0_CLIENT_ID,
        serialized.as_bytes(),
    )?;

    // if existing.is_ok() {
    //     // let mut entry = Entry::for_item(existing.unwrap());
    //     // entry.set_password(&serialized)?;
    // } else {
    //     // let mut entry = Entry::new("parra_cli", AUTH0_CLIENT_ID)?;
    //     // entry.set_password(&serialized)?;
    // }

    // let entry = Entry::new("parra_cli", "auth")?;

    // entry.entry.set_password(&serialized)?;

    Ok(())
}

async fn post_form_request<T: DeserializeOwned>(
    url: &str,
    fields: Vec<(String, String)>,
) -> Result<T, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.post(url).form(&fields).send().await?;

    let status = response.status();
    let body = response.text().await?;

    if status.is_success() {
        return Ok(serde_json::from_str::<T>(&body)?);
    } else {
        eprintln!("Request failed with status {}: {}", status, body);
        return Err("Request failed".into());
    }
}

async fn poll_for_token<T: DeserializeOwned>(
    url: &str,
    interval: u64,
    expires_in: u64,
    body: TokenRequest,
) -> Result<T, Box<dyn Error>> {
    let interval = Duration::from_secs(interval);
    let start_time = Instant::now();
    let expires_in = Duration::from_secs(expires_in);

    let client = reqwest::Client::new();

    loop {
        // spec says to wait for the interval before the first poll
        async_std::task::sleep(interval).await;

        if start_time.elapsed() >= expires_in {
            eprintln!("Time expired.");

            return Err("Parra sign in request has expired. Try again.".into());
        }

        let response = client.post(url).json(&body).send().await?;
        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            return Ok(serde_json::from_str::<T>(&body)?);
        } else if status.as_u16() == 403 {
            println!("Waiting for user to confirm login...");
        } else {
            eprintln!(
                "Check for authorization token failed unexpectedly {}: {}",
                status, body
            );

            return Err("Request failed".into());
        }
    }
}
