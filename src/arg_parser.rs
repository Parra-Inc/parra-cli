use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'a', long = "application-id")]
    pub application_id: Option<String>,

    #[arg(short = 't', long = "tenant-id")]
    pub tenant_id: Option<String>,

    #[arg(short = 'n', long = "project-name")]
    pub project_name: Option<String>,

    #[arg(short = 'p', long = "project-path")]
    pub project_path: Option<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
