use crate::types::{auth::AuthInfo, project::ProjectInfo};

pub fn generate_xcode_project(auth_info: AuthInfo, project_info: ProjectInfo) {
    println!("Generating Xcode project with auth info: {:?}", auth_info);
    println!(
        "Generating Xcode project with project info: {:?}",
        project_info
    );
}
