use crate::types::AuthResponse;
// use crate::auth_server;
use crate::auth_server;
use std::sync::mpsc;
use std::thread::JoinHandle;
use url::form_urlencoded;

pub fn authenticate_user_async(
    tx: mpsc::Sender<AuthResponse>,
    application_id: Option<String>,
    application_name: Option<String>,
    tenant_id: Option<String>,
) -> JoinHandle<()> {
    let params: Vec<_> = [
        ("application_id", application_id),
        ("application_name", application_name),
        ("tenant_id", tenant_id),
    ]
    .into_iter()
    .filter_map(|(item, value)| value.map(|v| (item, v)))
    .collect();

    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params.iter())
        .finish();

    let result = open::that(format!("https://parra.io/auth/cli?{}", encoded));

    if result.is_err() {
        println!("Failed to open the browser. Please visit https://parra.io/auth/cli to authenticate.");
    }

    return auth_server::wait_for_auth_callback(tx);
}
