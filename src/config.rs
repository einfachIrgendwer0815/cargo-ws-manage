//! This module contains all settings for the argument parsing done by clap.

use clap::{AppSettings, Args, Parser, Subcommand};

/// This is the main configuration struct.
///
#[derive(Parser)]
#[clap(setting = AppSettings::NoBinaryName)]
#[clap(version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, action)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Commands,
}

/// This enum contains all possible sub-commands, which are
/// all represented by their own struct.
#[derive(Subcommand)]
pub enum Commands {
    ///Create new workspace
    New(New),
}

/// Stores futher arguments for the sub-command 'new'
#[derive(Args)]
pub struct New {
    /// Name of the project
    #[clap(value_parser)]
    pub project_name: String,

    /// Name of directory the workspace should be created in
    #[clap(value_parser)]
    pub directory_name: Option<String>,
}
