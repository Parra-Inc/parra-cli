use std::path::PathBuf;
use std::process::Command;
use std::{error::Error, fs};

use crate::{
    project_generator::{renderer, templates},
    types::api::{ApplicationResponse, TenantResponse},
};

pub fn generate_xcode_project(
    path: &PathBuf,
    _tenant: TenantResponse,
    application: ApplicationResponse,
) -> Result<(), Box<dyn Error>> {
    let app_name = application.name.clone();
    let bundle_id = application.ios.unwrap().bundle_id;

    create_project_structure(path, &app_name)?;

    let globals = liquid::object!({
        "app": {
            "name": app_name,
            "bundle_id": bundle_id,
        },
    });

    let project_yaml = renderer::render_template(
        &templates::get_project_yaml_template(),
        &globals,
    )
    .unwrap();

    run_xcodegen(path, &app_name, &project_yaml)?;

    println!("Project YAML: {}", project_yaml);

    Ok(())
}

pub fn create_project_structure(
    path: &PathBuf,
    app_name: &str,
) -> Result<(), Box<dyn Error>> {
    // Double app name directories. Outer is the repo/project level, inner is the main app target.
    let project_dir = path.join(app_name).join(app_name);

    fs::create_dir_all(project_dir)?;

    Ok(())
}

pub fn run_xcodegen(
    path: &PathBuf,
    app_name: &str,
    template: &str,
) -> Result<(), Box<dyn Error>> {
    fs::write(path.join("project.yml"), template)?;

    let output = Command::new("xcodegen")
        .arg("--spec")
        .arg(path.join("project.yml"))
        .arg("--project")
        .arg(app_name)
        .arg("--project-root")
        .arg(app_name)
        .output()?;

    println!("xcodegen output: {:?}", output);

    fs::remove_file(path.join("project.yml"))?;

    Ok(())
}
