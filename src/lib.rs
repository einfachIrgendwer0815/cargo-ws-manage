use config::{Commands, Config};

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
