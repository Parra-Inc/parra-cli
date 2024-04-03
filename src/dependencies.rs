use crate::types::dependency::XcodeVersion;
use semver::{Version, VersionReq};
use std::process::Stdio;
use std::{process::Command, sync::mpsc::Sender};

#[derive(Debug, PartialEq)]
pub enum DerivedDependency {
    Xcode,
}

// Brew:
// xcodes, aria2, xcodegen

// Need to get any information we will need from the user up front to install any deps
// Login for Xcodes, sudo password, etc.
// Dependencies then install in the background and the auth flow commences.

/// Primary dependencies such as xcodes, xcodegen, aria2, etc. being present will be enforced
/// by listing them as dependencies of the Homebrew formula. This function will check
/// derived dependencies such as Xcode versions.
pub fn install_missing_dependencies(
    tx: Sender<Vec<DerivedDependency>>,
    desired_xcode_version: XcodeVersion,
) {
    install_xcode(desired_xcode_version);

    let _ = tx.send(Vec::<DerivedDependency>::new());
}

pub fn check_for_missing_dependencies(
    min_xcode_version: XcodeVersion,
) -> Vec<DerivedDependency> {
    println!("Checking for missing dependencies");

    update_xcodes_list();

    let mut missing_deps = Vec::<DerivedDependency>::new();

    let valid_version = check_xcode_version(min_xcode_version);
    if !valid_version {
        missing_deps.push(DerivedDependency::Xcode)
    }

    return missing_deps;
}

fn check_xcode_version(min_version: XcodeVersion) -> bool {
    let output = Command::new("xcodes")
        .arg("installed")
        .arg("--no-color")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed. Error:\n{}", stderr);

        return false;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let min_version_string = format!(
        "{}.{}.{}",
        min_version.major, min_version.minor, min_version.patch
    );

    let min_version_req =
        match VersionReq::parse(&format!(">={}", min_version_string)) {
            Ok(req) => req,
            Err(_) => return false, // Return false if the minimum version string is invalid
        };

    for line in stdout.lines() {
        // Attempt to extract the version number from the beginning of the line
        if let Some(version_str) = line.split_whitespace().next() {
            let full_version_str = ensure_full_semver(version_str);

            // Parse the version string into a `Version`
            if let Ok(version) = Version::parse(&full_version_str) {
                // Check if the version is greater than or equal to the minimum version
                if min_version_req.matches(&version) {
                    println!("Found installed Xcode version: {}", version);
                    return true;
                }
            }
        }
    }

    println!("No installed Xcode version meets the minimum requirement");

    return false;
}

fn update_xcodes_list() {
    println!("Updating list of available Xcode versions");

    let output = Command::new("xcodes")
        .arg("update")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed. Error:\n{}", stderr);
    }
}

fn install_xcode(version: XcodeVersion) {
    let version_string =
        format!("{}.{}.{}", version.major, version.minor, version.patch);
    let version_string_clone = version_string.clone();

    println!("Installing Xcode version: {:?}", version_string);

    //  and if fail retry without

    let output = Command::new("xcodes")
        .arg("install")
        .arg(version_string)
        // Will skip prompting for password but will result in user being prompted for password to install
        // additional tools when they launch Xcode. This will likely be more streamlined.
        .arg("--no-superuser")
        .arg("--experimental-unxip")
        // Need to inherit stdio to allow the user to enter credentials when prompted by xcodes.
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed. Error:\n{}", stderr);
    }

    println!("Successfully installed Xcode: {}", version_string_clone);
}

fn ensure_full_semver(version: &str) -> String {
    // Check if the version string already has two dots
    if version.matches('.').count() < 2 {
        // If not, assume it's missing the patch version and append ".0"
        format!("{}.0", version)
    } else {
        version.to_string()
    }
}
