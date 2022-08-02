//! Manage cargo workspaces.
//!
//! # Creating a new workspace
//!
//! ```bash
//! cargo ws-manage new <PROJECT_NAME> [DIR_NAME]
//! ```
//! for example:
//! ```bash
//! cargo ws-manage new demo # creates new workspace in ./demo
//! cargo ws-manage new demo other_name # creates workspace in ./other_name
//! ```
//!
//! ## More in the future
//! There is not much else to see currently, but these are some of the features that are to be added in the future:
//!
//!- add crates
//!- remove crates
//!- pass cargo commands to all crates (like cargo fmt, cargo add, cargo publish, ...)
//!

use config::{Commands, Config};

/// Runs subcommands based on the given configuration
pub fn run(cfg: Config) {
    match &cfg.command {
        Commands::New(new_cfg) => new::run(new_cfg),
    }
}

pub mod config;
mod crates;
mod fs;
mod input;
mod new;
mod workspace;
