use clap::{Parser, AppSettings};

#[derive(Parser)]
#[clap(setting = AppSettings::NoBinaryName)]
#[clap(version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, action)]
    pub verbose: bool,
}
