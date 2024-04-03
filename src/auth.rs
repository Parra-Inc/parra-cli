use keyring::Entry;
use rouille::{Response, ResponseBody};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;
use std::io::Read;
use std::sync::mpsc;
use std::thread;
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
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    /// lifetime in seconds for device_code and user_code
    pub expires_in: u64,
    /// The interval to poll the token endpoint
    pub interval: u64,
}

#[derive(Debug, Serialize)]
pub struct TokenRequest {
    // pub grant_type: String,
    // pub client_id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TokenResponse {}

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

pub async fn perform_device_authentication() {
    let device_code_url = "https://parra.auth0.com/oauth/device/code";

    let device_auth_response: Result<DeviceAuthResponse, Box<dyn Error>> =
        post_form_request(
            device_code_url,
            vec![
                (
                    "client_id".to_string(),
                    "nD9GTUvvqCT0oWi34L2IdJiK0YjupSjY".to_string(),
                ),
                // ("scope".to_string(), "openid profile email".to_string()),
            ],
        )
        .await;

    println!("Device auth response: {:?}", device_auth_response);
    // device_code: "OqIJekqwmzBgGkSXHPZQDeuZ",
    // user_code: "DBPZ-TSGF",
    // verification_uri: "https://parra.auth0.com/activate",
    // verification_uri_complete: "https://parra.auth0.com/activate?user_code=DBPZ-TSGF",
    // expires_in: 900,
    // interval: 5
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
        // client_id: "nD9GTUvvqCT0oWi34L2IdJiK0YjupSjY".to_string(),
        // device_code: device_auth.device_code,
        // grant_type: "urn:ietf:params:oauth:grant-type:device_code".to_string(),
    };

    let poll_result: TokenResponse = poll_for_token(
        token_url,
        device_auth.interval,
        device_auth.expires_in,
        token_request_body,
    )
    .await
    .unwrap();
}

fn store_struct(
    service: &str,
    account: &str,
    data: &AuthResponse,
) -> keyring::Result<()> {
    let serialized = serde_json::to_string(data).unwrap();

    // let keyring = keyring::Entry::new(service, account);

    // keyring.set_password(&serialized)?;

    let entry = Entry::new("parra_cli", "auth")?;
    entry.set_password(&serialized)?;

    Ok(())
}

fn open_url_and_wait_for_callback<
    T: DeserializeOwned + Clone + Debug + Send + 'static,
>(
    url: &str,
    expected_route: String,
) -> T {
    let (tx, rx) = mpsc::channel::<T>();

    let result = open::that(url);

    if result.is_err() {
        println!("Failed to open the browser. Please visit {}", url);
    }

    thread::spawn(move || {
        rouille::start_server("localhost:7272", move |request| {
            let method = request.method();
            let url = request.url();

            if url != format!("/{}", expected_route) {
                eprintln!("404: {} {}", method, url);
                return Response::empty_404();
            }

            if !method.eq_ignore_ascii_case("post") {
                eprintln!("405: {} {}", method, url);

                return Response {
                    status_code: 405,
                    headers: vec![],
                    data: ResponseBody::empty(),
                    upgrade: None,
                };
            };

            println!("Request: {:?}", request);

            let mut body = String::new();

            // Attempt to read the request body directly into the String
            if let Some(mut data) = request.data() {
                match data.read_to_string(&mut body) {
                    Ok(_) => {
                        // Parse the URL-encoded string
                        match serde_urlencoded::from_str::<T>(&body) {
                            Ok(form) => {
                                println!("Parsed form data: {:?}", form);
                                Response::text("Parra login successful! You can close this window and return to the terminal.")
                                    .with_status_code(200)
                            }
                            Err(e) => {
                                eprintln!("Failed to parse form data: {}", e);
                                Response::text("Failed to parse form data")
                                    .with_status_code(400)
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read request body: {}", e);
                        return Response::text("Failed to read request data")
                            .with_status_code(400);
                    }
                }
            } else {
                return Response::text("Failed to access request data")
                    .with_status_code(400);
            }
        });
    });

    return rx.recv().unwrap();
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

        println!("Polling for token...");

        if start_time.elapsed() >= expires_in {
            eprintln!("Time expired.");

            return Err("Parra sign in request has expired. Try again.".into());
        }

        let response = client.post(url).json(&body).send().await?;
        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            return Ok(serde_json::from_str::<T>(&body)?);
        } else {
            eprintln!("Request failed with status {}: {}", status, body);
            return Err("Request failed".into());
        }
    }
}
