mod arg_parser;
mod auth;
mod auth_server;
mod dependencies;
mod project_generator;
mod types;
use std::error::Error;
use std::io::{self, Write};
use std::process::exit;
use std::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = arg_parser::parse_args();

    let has_application_id = args.application_id.is_some();
    let has_tenant_id = args.tenant_id.is_some();
    let has_project_name = args.project_name.is_some();

    let min_xcode_version = types::XcodeVersion {
        major: 15,
        minor: 3,
        patch: 0,
    };

    let desired_xcode_version = types::XcodeVersion {
        major: 15,
        minor: 3,
        patch: 0,
    };

    let requires_auth =
        !has_application_id || !has_tenant_id || !has_project_name;
    let missing =
        dependencies::check_for_missing_dependencies(min_xcode_version);
    let requires_dependencies = !missing.is_empty();

    let (tx_deps, rx_deps) =
        mpsc::channel::<Vec<dependencies::DerivedDependency>>();
    // let (tx_auth, rx_auth) = mpsc::channel::<types::AuthResponse>();

    if requires_dependencies {
        if missing.contains(&dependencies::DerivedDependency::Xcode) {
            print!("We need to install a few dependencies, including an Xcode update. You will be prompted to enter your Apple ID and password in order to download it. Proceed? (y/n): ");
        } else {
            print!(
                "We need to install a few dependencies first. Proceed? (y/n): "
            );
        }

        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        // Read the user's input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim the input and check if it's an affirmative response
        if input.trim().eq_ignore_ascii_case("y") {
            println!("User confirmed action.");

            dependencies::install_missing_dependencies(
                tx_deps,
                desired_xcode_version,
            );

            rx_deps.recv().unwrap();
        } else {
            println!("Parra bootstrap cancelled. ");
            exit(1)
        }
    }

    // auth::perform_authentication()
    auth::perform_device_authentication().await?;

    Ok(())

    // let (auth_server_handle, auth_response): (
    //     Option<JoinHandle<()>>,
    //     types::AuthResponse,
    // ) = if requires_auth {
    //     println!("+++++++ 4");
    //     let handle = auth::authenticate_user_async(
    //         tx_auth,
    //         args.application_id.clone(),
    //         args.project_name.clone(),
    //         args.tenant_id.clone(),
    //     );
    //     let auth_info = rx_auth.recv().unwrap();

    //     (Some(handle), auth_info)
    // } else {
    //     println!("+++++++ 5");
    //     (
    //         None,
    //         types::AuthResponse {
    //             application_id: args.application_id.unwrap(),
    //             application_name: args.project_name.unwrap(),
    //             tenant_id: args.tenant_id.unwrap(),
    //         },
    //     )
    // };

    // println!("+++++++ 6");

    // let auth_info = types::AuthInfo {
    //     application_id: auth_response.application_id,
    //     tenant_id: auth_response.tenant_id,
    // };

    // let project_info = types::ProjectInfo {
    //     name: auth_response.application_name,
    //     path: args.project_path.unwrap(),
    // };

    // println!("+++++++ 7");
    // // Generate project.
    // // Name and path?
    // project_generator::generate_xcode_project(auth_info, project_info);
}
