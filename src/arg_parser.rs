use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "")]
pub struct Args {
    /// The identifier of the application you want to bootstrap. You can find
    /// this value here: https://parra.io/dashboard/applications
    /// If you don't provide this value, you will be prompted to select an
    /// application or create a new one.
    #[arg(short = 'a', long = "application-id")]
    pub application_id: Option<String>,

    /// The identifier of the workspace that owns your application in the Parra
    /// dashboard. You can find this value here: https://parra.io/dashboard/settings
    /// If you don't provide this value, you will be prompted to select a workspace or
    /// create a new one.
    #[arg(short = 'w', long = "workspace-id")]
    pub workspace_id: Option<String>,

    /// The path where you want to create your project. If you don't provide this
    /// value, you will be prompted to enter a path.
    #[arg(short = 'p', long = "project-path")]
    pub project_path: Option<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
