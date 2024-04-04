mod api;
mod arg_parser;
mod auth;
mod dependencies;
mod project_generator;
mod types;
use inquire::validator::{MinLengthValidator, Validation};
use inquire::{Confirm, InquireError, Select, Text};
use regex::Regex;
use slugify::slugify;
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::io::{self, Write};
use std::process::exit;
use std::sync::mpsc;
use types::api::{ApplicationResponse, TenantResponse};
use types::dependency::XcodeVersion;

use crate::types::auth::AuthInfo;
use crate::types::project::ProjectInfo;

impl Display for TenantResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}

impl Display for ApplicationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(bundle_id) = &self.bundle_id {
            write!(f, "{} ({})", self.name, bundle_id)
        } else {
            write!(f, "{}", self.name)
        }
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

    let tenant = get_tenant(args.tenant_id).await?;
    let application = get_application(args.application_id, &tenant).await?;
    let relative_path = get_project_path(args.project_path);

    let auth_info = AuthInfo {
        application_id: application.id,
        tenant_id: tenant.id,
    };

    let current_dir = env::current_dir()?;
    let project_path = current_dir.join(relative_path);

    let final_path = project_path.to_str().unwrap();

    let project_info = ProjectInfo {
        name: application.name,
        path: final_path.to_string(),
    };

    project_generator::generator::generate_xcode_project(
        auth_info,
        project_info,
    );

    Ok(())
}

async fn get_tenant(
    tenant_arg: Option<String>,
) -> Result<TenantResponse, Box<dyn Error>> {
    // The user provided a tenant ID directly.
    if let Some(tenant_id) = tenant_arg {
        return api::get_tenant(&tenant_id).await;
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

        return Ok(selected_tenant?);
    } else {
        return create_new_tenant().await;
    }
}

async fn get_application(
    application_arg: Option<String>,
    tenant: &TenantResponse,
) -> Result<ApplicationResponse, Box<dyn Error>> {
    // The user provided a application ID directly.
    if let Some(application_arg) = application_arg {
        return api::get_application(&tenant.id, &application_arg).await;
    }

    let applications = api::paginate_applications(&tenant.id).await?;

    if applications.is_empty() {
        return create_new_application(&tenant).await;
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

        match selected_application {
            Ok(application) => return Ok(application),
            Err(error) => Err(error.into()),
        }
    } else {
        return create_new_application(&tenant).await;
    }
}

fn get_project_path(project_path_arg: Option<String>) -> String {
    if let Some(project_path) = project_path_arg {
        return project_path;
    }

    let project_path =
        Text::new("Where would you like to create your project?")
            .with_default("./")
            .with_validator(MinLengthValidator::new(1))
            .with_help_message("Provide a relative path to the directory where you would like to create your project. A new directory will be created in this location with the name of your application.")
            .prompt()
            .unwrap();

    return project_path;
}

async fn create_new_tenant() -> Result<TenantResponse, Box<dyn Error>> {
    let name = Text::new(
        "No existing tenants found. What would you like to call your tenant?",
    )
    .with_validator(MinLengthValidator::new(1))
    .prompt()?;

    return api::create_tenant(&name.trim()).await;
}

async fn create_new_application(
    tenant: &TenantResponse,
) -> Result<ApplicationResponse, Box<dyn Error>> {
    let name = Text::new("What would you like to call your application?")
        .with_validator(MinLengthValidator::new(1))
        .prompt()?;

    let tenant_slug = slugify!(&tenant.name);
    let app_name_slug = slugify!(&name);

    let suggested_bundle_id = format!("com.{}.{}", tenant_slug, app_name_slug);

    let bundle_id = Text::new("What would you like your bundle ID to be?")
        .with_default(&suggested_bundle_id)
        .with_validator(MinLengthValidator::new(5)) // min for x.y.z
        .with_validator(|input: &str| {
            let re =
                Regex::new(r"^[a-zA-Z0-9-]+(\.[a-zA-Z0-9-]+){2,}$").unwrap();

            if re.is_match(input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("The bundle ID string must contain only alphanumeric characters (A–Z, a–z, and 0–9), hyphens (-), and periods (.). Typically, you use a reverse-DNS format for bundle ID strings. Bundle IDs are case-insensitive.".into()))
            }
        })
        .prompt()?;

    let new_application =
        api::create_application(&tenant.id, &name.trim(), &bundle_id.trim())
            .await?;

    return Ok(new_application);
}
