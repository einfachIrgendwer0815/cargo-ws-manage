use crate::crates::Crate;
use crate::fs::{self, CargoToml, PackageSection, WorkspaceSection};
use crate::input;

/// This struct creates a new Workspace with crates and
/// a workspace-level Cargo.toml, based on user input.
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Workspace {
    pub project_name: String,
    pub directory_name: String,
    pub root_crate: Option<Crate>,
    pub crates: Vec<Crate>,
}

impl Workspace {
    pub fn new(project_name: &String, directory_name: &Option<String>) -> Workspace {
        Workspace {
            project_name: project_name.clone(),
            directory_name: if let Some(name) = directory_name {
                name.clone()
            } else {
                project_name.clone()
            },
            root_crate: None,
            crates: Vec::new(),
        }
    }

    pub fn fill_from_user_input(&mut self) {
        self.root_crate = {
            if input::prompt_yes_no("Add root crate?", input::DefaultBool::YES).unwrap() {
                println!("\nPlease specify some information about the root crate:");
                let r_crate = Some(Crate::new_from_user_input(true, false, true));
                println!("");
                r_crate
            } else {
                None
            }
        };

        let mut crates = Vec::new();
        while input::prompt_yes_no(
            "Do you want to add a/another member crate?",
            input::DefaultBool::YES,
        )
        .unwrap()
        {
            println!("\nPlease specify some information about this crate:");
            crates.push(Crate::new_from_user_input(false, true, true));
            println!("");
        }

        self.crates = crates;
    }

    pub fn write_to_disk(&self) {
        fs::create_dir_or_handle_error(&self.directory_name);

        let mut deps = Vec::<String>::new();
        let mut members = Vec::<String>::new();

        for member_crate in &self.crates {
            member_crate.write_to_disk(&self.directory_name);

            members.push(member_crate.crate_name.clone());
            if member_crate.as_dependency == true {
                deps.push(member_crate.crate_name.clone());
            }
        }

        self.write_root_crate(&deps, &members);
    }

    fn write_root_crate(&self, deps: &Vec<String>, members: &Vec<String>) {
        if let Some(c) = &self.root_crate {
            c.write_to_disk(&self.directory_name);
        }

        fs::write_cargo_toml_or_handle_error(
            &self.directory_name,
            &CargoToml {
                package: if self.root_crate.is_some() {
                    Some(PackageSection {
                        pkg_name: &self.project_name,
                        pkg_version: "0.1.0",
                        pkg_edition: "2021",
                    })
                } else {
                    None
                },
                dependencies: if self.root_crate.is_some() {
                    Some(deps)
                } else {
                    None
                },
                workspace: Some(WorkspaceSection { members: members }),
            },
        );
    }
}
