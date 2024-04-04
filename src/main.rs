mod api;
mod arg_parser;
mod auth;
mod dependencies;
mod project_generator;
mod types;
use inquire::{Confirm, InquireError, Select, Text};
use std::error::Error;
use std::fmt::Display;
use std::io::{self, Write};
use std::process::exit;
use std::sync::mpsc;
use types::api::{ApplicationResponse, TenantResponse};
use types::dependency::XcodeVersion;

impl Display for TenantResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}

impl Display for ApplicationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = arg_parser::parse_args();

    // let has_project_name = args.project_name.is_some();

    let min_xcode_version = XcodeVersion {
        major: 15,
        minor: 3,
        patch: 0,
    };

    let desired_xcode_version = XcodeVersion {
        major: 15,
        minor: 3,
        patch: 0,
    };

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

    let tenant_id = get_tenant_id(args.tenant_id).await?;
    let application_id =
        get_application_id(args.application_id, &tenant_id).await?;

    println!("Tenant ID: {}", tenant_id);
    println!("Application ID: {}", application_id);

    Ok(())

    // let auth_info = types::AuthInfo {
    //     application_id: auth_response.application_id,
    //     tenant_id: auth_response.tenant_id,
    // };

    // let project_info = types::ProjectInfo {
    //     name: auth_response.application_name,
    //     path: args.project_path.unwrap(),
    // };
    // // Name and path?
    // project_generator::generate_xcode_project(auth_info, project_info);
}

async fn get_tenant_id(
    tenant_arg: Option<String>,
) -> Result<String, Box<dyn Error>> {
    // The user provided a tenant ID directly.
    if let Some(tenant_id) = tenant_arg {
        return Ok(tenant_id);
    }

    let tenants = api::get_tenants().await?;

    if tenants.is_empty() {
        return create_new_tenant().await;
    }

    let use_existing =
        Confirm::new("Would you like to use an existing tenant?")
            .with_default(true)
            .prompt()?;

    if use_existing {
        let selected_tenant: Result<TenantResponse, InquireError> =
            Select::new("Which tenant are you building an app for?", tenants)
                .prompt();

        return Ok(selected_tenant?.id);
    } else {
        return create_new_tenant().await;
    }
}

async fn get_application_id(
    application_arg: Option<String>,
    tenant_id: &str,
) -> Result<String, Box<dyn Error>> {
    // The user provided a application ID directly.
    if let Some(application_arg) = application_arg {
        return Ok(application_arg);
    }

    let applications = api::paginate_applications(tenant_id).await?;

    if applications.is_empty() {
        return create_new_application(tenant_id).await;
    }

    let use_existing = Confirm::new("Would you like to use an existing application?")
        .with_default(true)
        .with_help_message("We found existing applications that you can use. If you choose not to use them, a new application will be created.")
        .prompt()?;

    if use_existing {
        let selected_application: Result<ApplicationResponse, InquireError> =
            Select::new(
                "Which application are you building an app for?",
                applications,
            )
            .prompt();

        Ok(selected_application?.id)
    } else {
        return create_new_application(tenant_id).await;
    }
}

async fn create_new_tenant() -> Result<String, Box<dyn Error>> {
    let name = Text::new(
        "No existing tenants found. What would you like to call your tenant?",
    )
    .prompt()?;

    let new_tenant = api::create_tenant(&name).await;

    return Ok(new_tenant?.id);
}

async fn create_new_application(
    tenant_id: &str,
) -> Result<String, Box<dyn Error>> {
    let name =
        Text::new("What would you like to call your application?").prompt()?;

    let new_application = api::create_application(tenant_id, &name).await;

    return Ok(new_application?.id);
}
