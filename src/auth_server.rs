use crate::types;
use rouille::{Request, Response, ResponseBody};
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;
use std::{result, thread};

pub fn wait_for_auth_callback(
    tx: mpsc::Sender<types::AuthResponse>,
) -> thread::JoinHandle<()> {
    // Start an http server in a background thread. If a trigger endpoint receives a request, send a signal to the main thread.
    let server_thread = thread::spawn(move || {
        // TODO: Delete me
        sleep(Duration::from_secs(5));

        let _ = tx.send(types::AuthResponse {
            application_id: "123".to_string(),
            application_name: "Parra".to_string(),
            tenant_id: "456".to_string(),
        });

        rouille::start_server("localhost:7272", move |request| {
            println!("Request: {:?}", request);

            if request.url() == "/trigger" {
                let _ = &tx.send(types::AuthResponse {
                    application_id: "123".to_string(),
                    application_name: "Parra".to_string(),
                    tenant_id: "456".to_string(),
                });

                Response {
                    status_code: 200,
                    headers: vec![],
                    data: ResponseBody::empty(),
                    upgrade: None,
                }
            } else {
                Response::empty_400()
            }
        });
    });

    return server_thread;
}

// println!("Welcome to Parra!");

// // Start HTTP server in a background thread
// let server_thread = thread::spawn(move || {
//     rouille::start_server("localhost:7272", move |request| {
//         println!("Request: {:?}", request);
//         // Logic to handle specific requests
//         if request.url() == "/trigger" {
//             // Send signal to main thread
//             let _ = tx.send(types::AuthInfo {
//                 application_id: "123".to_string(),
//                 tenant_id: "456".to_string(),
//                 token: "789".to_string(),
//             });

//             Response {
//                 status_code: 200,
//                 headers: vec![],
//                 data: ResponseBody::empty(),
//                 upgrade: None,
//             }
//         } else {
//             Response::empty_400()
//         }
//     });
// });

// // Setup Clap and parse CLI input (omitted for brevity)

// // Wait for the signal from the HTTP server
// println!("Waiting for the trigger...");
// rx.recv().unwrap();

// println!("Trigger received, proceeding with the flow...");

// // Ensure the server thread finishes (e.g., by sending it a shutdown signal or joining it)
// server_thread.join().unwrap(); // Example, depends on your shutdown logic
