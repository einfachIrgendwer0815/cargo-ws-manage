//! This module contains all settings for the argument parsing done by clap.

use clap::{AppSettings, Args, Parser, Subcommand};

pub mod new;

pub use new::*;

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
