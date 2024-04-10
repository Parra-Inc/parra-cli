use crate::types::api::{ApplicationResponse, TenantResponse};
use crate::types::dependency::XcodeVersion;
use crate::{api, dependencies, project_generator};
use convert_case::{Case, Casing};
use inquire::validator::{MaxLengthValidator, MinLengthValidator, Validation};
use inquire::{Confirm, InquireError, Select, Text};
use regex::Regex;
use slugify::slugify;
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::path::Path;
use std::process::exit;

static MIN_XCODE_VERSION: XcodeVersion = XcodeVersion {
    major: 15,
    minor: 3,
    patch: 0,
};

static DESIRED_XCODE_VERSION: XcodeVersion = XcodeVersion {
    major: 15,
    minor: 3,
    patch: 0,
};

impl Display for TenantResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}

impl Display for ApplicationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ios) = &self.ios {
            write!(f, "{} ({})", self.name, ios.bundle_id)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

pub async fn execute_bootstrap(
    application_id: Option<String>,
    workspace_id: Option<String>,
    project_path: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let tenant = get_tenant(workspace_id).await?;
    let mut application = get_application(application_id, &tenant).await?;

    // If the app name ends with "App", remove it.
    if application.name.to_lowercase().ends_with("app") {
        application.name =
            application.name.trim_end_matches("app").trim().to_string();
    }

    let kebab_name = application.name.to_case(Case::Kebab);
    let relative_path = get_project_path(project_path, &kebab_name);

    let current_dir = env::current_dir()?;
    let supplied = Path::new(&relative_path);
    let mut project_path = current_dir.join(supplied);

    println!("Creating project at {}", project_path.display());

    if !project_path.ends_with(&kebab_name) {
        project_path = project_path.join(&kebab_name);
    }

    println!("Generating project...");

    project_generator::generator::generate_xcode_project(
        &current_dir,
        &project_path,
        tenant,
        application,
    )?;

    println!("🚀 Parra project generated at {}!", relative_path);

    dependencies().await?;

    Ok(())
}

async fn dependencies() -> Result<(), Box<dyn Error>> {
    let missing =
        dependencies::check_for_missing_dependencies(MIN_XCODE_VERSION);
    let requires_dependencies = !missing.is_empty();

    if requires_dependencies {
        let confirm_message = if missing
            .contains(&dependencies::DerivedDependency::Xcode)
        {
            "We need to install a few dependencies, including an Xcode update. You will be prompted to enter your Apple ID and password in order to download it. Proceed?"
        } else {
            "We need to install a few dependencies first. Proceed?"
        };

        let confirmed_install =
            Confirm::new(confirm_message).with_default(true).prompt()?;

        // Trim the input and check if it's an affirmative response
        if confirmed_install {
            dependencies::install_missing_dependencies(DESIRED_XCODE_VERSION);
        } else {
            println!("Parra bootstrap cancelled. ");
            exit(1)
        }
    }

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
        Confirm::new("Would you like to use an existing workspace?")
            .with_default(true)
            .prompt()?;

    if use_existing {
        let selected_tenant: Result<TenantResponse, InquireError> =
            Select::new("Please select a workspace", tenants).prompt();

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
            Select::new("Please select an application", applications).prompt();

        match selected_application {
            Ok(application) => return Ok(application),
            Err(error) => Err(error.into()),
        }
    } else {
        return create_new_application(&tenant).await;
    }
}

fn get_project_path(
    project_path_arg: Option<String>,
    app_name: &str,
) -> String {
    if let Some(project_path) = project_path_arg {
        return project_path;
    }

    let default_path = format!("./{}", app_name);
    let default_message = format!("For example: ~/Desktop/{}", app_name);

    let project_path =
        Text::new("Where would you like to create your project?")
            .with_default(&default_path)
            .with_validator(MinLengthValidator::new(1))
            .with_help_message(&default_message)
            .prompt()
            .unwrap();

    return project_path;
}

async fn create_new_tenant() -> Result<TenantResponse, Box<dyn Error>> {
    let name = Text::new(
        "No existing workspaces found. What would you like to call your workspace?",
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
        .with_validator(MaxLengthValidator::new(155))
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
