use crate::config::New;
use crate::fs;
use crate::input;
use crate::toml_data;
use std::io::ErrorKind;
use std::path::Path;

pub fn run(cfg: &New) {
    let dirname = if let Some(name) = &cfg.directory_name {
        &name
    } else {
        &cfg.project_name
    };

    create_workspace_directory(&dirname);

    let with_root = input::prompt_yes_no("Add a root crate?", input::DefaultBool::YES).unwrap();

    print!("Writing workspace Cargo.toml...");
    write_cargo_toml(&dirname, &cfg.project_name, with_root);
    println!("Done");

    if with_root == false {
        return;
    }

    print!("Init root crate...");
    write_root_crate(&dirname);
    println!("Done");
}

fn write_cargo_toml(dirname: &str, project_name: &str, root_crate: bool) {
    let data = toml_data::CargoToml {
        package: if root_crate == true {
            Some(toml_data::Package {
                name: String::from(project_name),
                version: String::from("0.1.0"),
                edition: String::from("2021"),
            })
        } else {
            None
        },
        workspace: toml_data::Workspace {},
    };

    let filename = format!("{}/Cargo.toml", dirname);
    let result = fs::write_toml_file(Path::new(&filename), &data, false);
    match result {
        Ok(_) => {}
        Err(e) => {
            println!("An error occurred while writing Cargo.toml: {:?}", e);
        }
    };
}

fn write_root_crate(dirname: &str) {
    let dirname = format!("{}/src", dirname);
    let filename = format!("{}/main.rs", dirname);

    match fs::mkdir(Path::new(&dirname), false) {
        Ok(_) => {}
        Err(e) => {
            panic!("While init root crate: {:?}", e);
        }
    };

    match fs::write_file(
        Path::new(&filename),
        &String::from(toml_data::DEFAULT_MAIN_RS),
        false,
    ) {
        Ok(_) => {}
        Err(e) => {
            panic!("While init root crate: {:?}", e);
        }
    };
}

fn create_workspace_directory(dirname: &str) {
    match fs::mkdir(Path::new(dirname), false) {
        Ok(_) => {}
        Err(e) => {
            match e {
                fs::IOError::InvalidPath => {
                    println!("Path {} is invalid", dirname);
                    std::process::exit(1);
                }
                fs::IOError::FsError(e) => {
                    match e.kind() {
                        ErrorKind::AlreadyExists => {
                            println!("Directory {} already exists", dirname);
                            std::process::exit(1);
                        }
                        _ => {
                            println!(
                                "Error occurred when creating directory {}: {:?}",
                                dirname, e
                            );
                            std::process::exit(1);
                        }
                    };
                }
                _ => {}
            };
        }
    };
}
