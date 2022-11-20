//! Subcommand 'new'
//!

use clap::Args;

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
