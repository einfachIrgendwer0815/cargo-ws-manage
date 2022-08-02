use crate::fs::{self, CargoToml, PackageSection};
use crate::input;
use std::path::Path;

/// This struct makes a directory for a crate and a Cargo.toml,
/// based on user input.
pub struct Crate {
    pub is_root: bool,
    pub crate_name: String,
    pub directory_name: String,
    pub as_dependency: bool,
    pub binary: bool,
}

impl Crate {
    pub fn new_from_user_input(is_root: bool, root_exists: bool, indent: bool) -> Crate {
        let mut crate_name = String::new();
        let mut directory_name = String::from(".");
        let mut as_dependency = false;

        let indent_str = "    ";
        let crate_name_prompt =
            { &format!("{}Name of the crate?", if indent { indent_str } else { "" }) };

        let directory_name_prompt = {
            &format!(
                "{}Name of the directory the crate will be in?",
                if indent { indent_str } else { "" }
            )
        };

        let as_dependency_prompt = {
            &format!(
                "{}Is this crate a dependency of the root crate?",
                if indent { indent_str } else { "" }
            )
        };

        let library_prompt = {
            &format!(
                "{}Is this crate a library crate?",
                if indent { indent_str } else { "" }
            )
        };

        if is_root == false {
            crate_name = input::get_string(crate_name_prompt, None, Some(false));
            directory_name =
                input::get_string(directory_name_prompt, Some(crate_name.clone()), None);

            if root_exists {
                as_dependency =
                    input::prompt_yes_no(as_dependency_prompt, input::DefaultBool::YES).unwrap();
            }
        }

        let binary = !input::prompt_yes_no(
            library_prompt,
            if is_root {
                input::DefaultBool::NO
            } else {
                input::DefaultBool::YES
            },
        )
        .unwrap();

        Crate {
            is_root,
            crate_name,
            directory_name,
            as_dependency,
            binary,
        }
    }

    pub fn write_to_disk(&self, root_dir: &String) {
        let dirname = format!("{}/{}", root_dir, self.directory_name);

        if self.is_root == false {
            fs::create_dir_or_handle_error(&dirname);
        }

        self.write_src(&dirname);

        if self.is_root == true {
            return;
        }

        fs::write_cargo_toml_or_handle_error(
            &dirname,
            &CargoToml {
                package: Some(PackageSection {
                    pkg_name: &self.crate_name,
                    pkg_version: "0.1.0",
                    pkg_edition: "2021",
                }),
                dependencies: None,
                workspace: None,
            },
        );
    }

    fn write_src(&self, crate_dirname: &String) {
        let src_dir = format!("{}/src", crate_dirname);
        fs::create_dir_or_handle_error(&src_dir);

        let filename = if self.binary == true {
            format!("{}/src/main.rs", crate_dirname)
        } else {
            format!("{}/src/lib.rs", crate_dirname)
        };

        let result = if self.binary == true {
            fs::write_file(
                &Path::new(&filename),
                &String::from(
                    "\
fn main() {
    println!(\"Hello, world!\");
}
",
                ),
                false,
            )
        } else {
            fs::write_file(
                &Path::new(&filename),
                &String::from(
                    "\
pub fn run() {
    println!(\"Hello, world!\");
}
",
                ),
                false,
            )
        };

        match result {
            Ok(_) => {}
            Err(e) => match e {
                fs::IOError::FsError(e) => {
                    println!("Error writing file {}: {}", filename, e);
                }
                _ => {
                    panic!("Unexpected error while writing file {}", filename);
                }
            },
        };
    }
}
