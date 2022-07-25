//! Manage cargo workspaces.
//!
//! There is not much to see currently, but these are some of the features that are to be added in the future:
//!
//!- create workspaces:
//!  - with and without root crates
//!- add crates
//!- remove crates
//!- pass cargo commands to all crates (like cargo fmt, cargo add, cargo publish, ...)
//!
//! # Examples:
//! Create a new workspace:
//! ```bash
//! cargo ws-manage new <PROJECT_NAME> [DIR_NAME]
//! ```
//! for example:
//! ```bash
//! cargo ws-manage new demo # creates new workspace in ./demo
//! cargo ws-manage new demo other_name # creates workspace in ./other_name
//! ```


use config::{Commands, Config};

/// Runs subcommands based on the given configuration
pub fn run(cfg: Config) {
    match &cfg.command {
        Commands::New(new_cfg) => new::run(new_cfg),
    }
}

pub mod config;
pub mod fs;
pub mod input;
mod new;
mod toml_data;
