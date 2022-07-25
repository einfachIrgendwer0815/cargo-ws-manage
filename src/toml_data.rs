use serde::{Deserialize, Serialize};

pub const DEFAULT_MAIN_RS: &str = "\
fn main() {
    println!(\"Hello, world!\");
}";

#[derive(Serialize, Deserialize, Debug)]
pub struct CargoToml {
    pub package: Option<Package>,

    pub workspace: Workspace,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {}
