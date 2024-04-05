use std::path::PathBuf;
use std::process::Command;
use std::{error::Error, fs};

use convert_case::{Case, Casing};

use crate::{
    project_generator::{renderer, templates},
    types::api::{ApplicationResponse, TenantResponse},
};

pub fn generate_xcode_project(
    path: &PathBuf,
    tenant: TenantResponse,
    application: ApplicationResponse,
) -> Result<(), Box<dyn Error>> {
    let app_name = application.name;
    let camel_name = app_name.to_case(Case::UpperCamel);
    let bundle_id = application.ios.unwrap().bundle_id;

    let project_dir = path.join(app_name.clone());
    let target_dir = project_dir.join(app_name.clone());

    create_project_structure(&target_dir)?;

    let globals = liquid::object!({
        "app": {
            "id": application.id,
            "name": app_name,
            "camel_name": camel_name,
            "bundle_id": bundle_id,
        },
        "tenant": {
            "id": tenant.id,
            "name": tenant.name,
        }
    });

    create_project_files(&target_dir, &camel_name, &globals)?;

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
    target_path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(target_path)?;

    Ok(())
}

pub fn create_project_files(
    target_path: &PathBuf,
    camel_app_name: &str,
    globals: &liquid::Object,
) -> Result<(), Box<dyn Error>> {
    let app_swift_yaml = renderer::render_template(
        &templates::get_app_swift_template(),
        &globals,
    )
    .unwrap();

    let app_content_view_yaml = renderer::render_template(
        &templates::get_content_view_swift_template(),
        &globals,
    )
    .unwrap();

    let app_path = target_path.join(format!("{}App.swift", camel_app_name));
    let content_view_path = target_path.join("ContentView.swift");

    fs::write(app_path, app_swift_yaml)?;
    fs::write(content_view_path, app_content_view_yaml)?;

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
