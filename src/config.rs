use clap::{AppSettings, Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(setting = AppSettings::NoBinaryName)]
#[clap(version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, action)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ///Create new workspace
    New(New),
}

#[derive(Args)]
pub struct New {
    /// Name of the project
    #[clap(value_parser)]
    pub project_name: String,

    /// Name of directory the workspace should be created in
    #[clap(value_parser)]
    pub directory_name: Option<String>,
}
