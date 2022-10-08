//! This module handles all i/o tasks.

use std::fs;
use std::io::{self, ErrorKind, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use toml::{self, value::Table, Value};

pub use cargo_toml::{CargoToml, PackageSection, WorkspaceSection};

pub mod cargo_toml;

/// This enum contains all possible erros raised by the cargo_ws_manage::fs module.
#[derive(Debug)]
pub enum IOError {
    InvalidPath,
    TomlSerError(toml::ser::Error),
    TomlDeError(toml::de::Error),
    FsError(io::Error),
}

/// Creates a directory.
pub fn mkdir(path: &Path, recursive: bool) -> Result<(), IOError> {
    let path_str = unpack_path(path)?;

    let result = match recursive {
        true => fs::create_dir_all(path_str),
        false => fs::create_dir(path_str),
    };

    if let Err(e) = result {
        return Err(IOError::FsError(e));
    }

    Ok(())
}

/// Deletes a directory.
pub fn rmdir(path: &Path, recursive: bool) -> Result<(), IOError> {
    let path_str = unpack_path(path)?;

    let result = match recursive {
        true => fs::remove_dir_all(path_str),
        false => fs::remove_dir(path_str),
    };

    if let Err(e) = result {
        return Err(IOError::FsError(e));
    }

    Ok(())
}

/// Reads a toml file and deserializes it into an object.
pub fn read_toml_file<T: for<'a> Deserialize<'a>>(path: &Path) -> Result<T, IOError> {
    let data = read_file(path)?;
    match toml::from_str::<T>(&data) {
        Ok(d) => Ok(d),
        Err(e) => {
            return Err(IOError::TomlDeError(e));
        }
    }
}

/// Reads a file to string.
pub fn read_file(path: &Path) -> Result<String, IOError> {
    let path_str = unpack_path(path)?;

    let mut file = match fs::File::open(path_str) {
        Ok(f) => f,
        Err(e) => {
            return Err(IOError::FsError(e));
        }
    };

    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => {
            return Err(IOError::FsError(e));
        }
    }
}

/// Serializes an object writes a toml file.
pub fn write_toml_file(
    path: &Path,
    data: &impl Serialize,
    allow_overwrite: bool,
) -> Result<(), IOError> {
    let toml = match toml::to_string(&data) {
        Ok(s) => s,
        Err(e) => {
            return Err(IOError::TomlSerError(e));
        }
    };

    write_file(path, &toml, allow_overwrite)?;
    Ok(())
}

/// Writes a file from a string.
pub fn write_file(path: &Path, data: &String, allow_overwrite: bool) -> Result<(), IOError> {
    let path_str = unpack_path(path)?;

    match fs::metadata(path_str) {
        Ok(m) => {
            if m.is_dir() {
                return Err(IOError::FsError(io::Error::new(
                    io::ErrorKind::Other,
                    "Is a directory",
                )));
            } else if m.is_symlink() {
                return Err(IOError::FsError(io::Error::new(
                    io::ErrorKind::Other,
                    "Is a symlink",
                )));
            } else if m.is_file() && !allow_overwrite {
                return Err(IOError::FsError(io::Error::from(
                    io::ErrorKind::AlreadyExists,
                )));
            }
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {}
            _ => {
                return Err(IOError::FsError(e));
            }
        },
    }

    let mut file = match fs::File::create(path_str) {
        Ok(f) => f,
        Err(e) => {
            return Err(IOError::FsError(e));
        }
    };

    match file.write(data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(IOError::FsError(e)),
    }
}

/// Unpacks the string of a path or raises an error if the path is invalid.
fn unpack_path(path: &Path) -> Result<&str, IOError> {
    match path.to_str() {
        Some(s) => Ok(s),
        None => {
            return Err(IOError::InvalidPath);
        }
    }
}

/// Takes a [CargoToml] instance, then generates an object structure that
/// is serializable by the [toml] library. The toml string is then written to a file.
/// If an error occures anywhere during that process, they'll get handled by
/// displaying an error message and exiting with code 1.
pub fn write_cargo_toml_or_handle_error(dirname: &str, content: &CargoToml) {
    let toml_object = gen_toml_object(content);
    let filename = format!("{}/Cargo.toml", dirname);

    match write_toml_file(&Path::new(&filename), &toml_object, false) {
        Ok(_) => {}
        Err(e) => {
            match e {
                IOError::TomlSerError(e) => {
                    println!(
                        "Writing {} failed while processing toml object with error: {}",
                        filename, e
                    );
                    std::process::exit(1);
                }
                IOError::FsError(e) => {
                    println!("Writing {} failed with error: {}", filename, e);
                    std::process::exit(1);
                }
                _ => {
                    panic!(
                        "Unexpected error occured when writing the toml file: {}",
                        filename
                    );
                }
            };
        }
    };
}

/// Generates a object that can be serialized by the [toml] library
/// out of an [CargoToml] instance.
fn gen_toml_object(content: &CargoToml) -> Table {
    let mut data = Table::new();

    if let Some(p) = &content.package {
        let mut pkg = Table::new();
        pkg.insert(
            String::from("name"),
            Value::String(String::from(p.pkg_name.clone())),
        );
        pkg.insert(
            String::from("version"),
            Value::String(String::from(p.pkg_version.clone())),
        );
        pkg.insert(
            String::from("edition"),
            Value::String(String::from(p.pkg_edition.clone())),
        );

        data.insert(String::from("package"), Value::Table(pkg));
    }

    if let Some(d) = &content.dependencies {
        let deps: Table = {
            let mut t = Table::new();
            for i in d.iter() {
                let mut inner_t = Table::new();
                inner_t.insert(String::from("path"), Value::String(i.clone()));
                t.insert(i.clone(), Value::Table(inner_t));
            }
            t
        };
        data.insert(String::from("dependencies"), Value::Table(deps));
    }

    if let Some(w) = &content.workspace {
        let ws: Table = {
            let mut t = Table::new();
            let members_as_values = w.members.iter().map(|i| Value::String(i.clone())).collect();
            t.insert(String::from("members"), Value::Array(members_as_values));
            t
        };
        data.insert(String::from("workspace"), Value::Table(ws));
    }

    data
}

/// Creates a directory by using [mkdir], but errors will be handled by
/// displaying an error message and exiting the process with code 1.
pub fn create_dir_or_handle_error(dirname: &str) {
    match mkdir(Path::new(dirname), false) {
        Ok(_) => {}
        Err(e) => {
            match e {
                IOError::InvalidPath => {
                    println!("Path {} is invalid", dirname);
                    std::process::exit(1);
                }
                IOError::FsError(e) => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpack_path_test() {
        let path = Path::new("/abc/def/123/X_y_z.txt");
        let path2 = Path::new("/elephant/🐘.txt");

        assert_eq!("/abc/def/123/X_y_z.txt", unpack_path(&path).unwrap());
        assert_eq!("/elephant/🐘.txt", unpack_path(&path2).unwrap());
    }
}

#[cfg(test)]
pub mod context_setup {
    use rand::{thread_rng, Rng};
    use std::fs;
    use std::io::ErrorKind;
    use test_context::TestContext;

    pub struct TestFiles {
        pub name: String,
    }

    impl TestContext for TestFiles {
        fn setup() -> TestFiles {
            let dir_name = format!("test_files_{}", thread_rng().gen_range(0..10000));
            match fs::create_dir(dir_name.as_str()) {
                Ok(_) => TestFiles {
                    name: dir_name.clone(),
                },
                Err(e) => match e.kind() {
                    ErrorKind::AlreadyExists => TestFiles {
                        name: dir_name.clone(),
                    },
                    _ => panic!("{}", e),
                },
            }
        }

        fn teardown(self) {
            if let Err(_) = fs::remove_dir_all(self.name) {};
        }
    }
}

#[cfg(test)]
mod test_dir {
    use super::context_setup::TestFiles;
    use super::*;
    use test_context::test_context;

    #[test_context(TestFiles)]
    #[test]
    fn test_mkdir(ctx: &mut TestFiles) {
        let raw_path = format!("{}/test_mkdir_not_recursive", &ctx.name);
        let path = Path::new(raw_path.as_str());

        let result = mkdir(&path, false);
        assert!(result.is_ok());
    }

    #[test_context(TestFiles)]
    #[test]
    fn test_mkdir_recursive(ctx: &mut TestFiles) {
        let raw_path = format!("{}/test_mkdir_recursive/inner_dir", &ctx.name);
        let path = Path::new(raw_path.as_str());

        let result = mkdir(&path, true);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_toml {
    use super::context_setup::TestFiles;
    use test_context::{test_context, TestContext};

    struct ReadTomlContext {
        test_files: TestFiles,
        pub filename: String,
    }

    impl TestContext for ReadTomlContext {
        fn setup() -> ReadTomlContext {
            let test_files = TestFiles::setup();
            let filename = format!("{}/toml_read.toml", test_files.name);

            let mut file = match fs::File::create(&filename) {
                Ok(f) => f,
                Err(e) => panic!("{}", e),
            };

            let data = "\
hello = \"world\"
name = \"Santa Claus\"

[section_a]
abc = \"CBA\"
xyz = 54626";

            match file.write(data.as_bytes()) {
                Ok(_) => {}
                Err(e) => panic!("{}", e),
            };

            ReadTomlContext {
                test_files,
                filename,
            }
        }

        fn teardown(self) {
            self.test_files.teardown();
        }
    }

    struct WriteTomlContext {
        test_files: TestFiles,
        pub filename: String,
    }

    impl TestContext for WriteTomlContext {
        fn setup() -> WriteTomlContext {
            let test_files = TestFiles::setup();
            let filename = format!("{}/toml_write.toml", test_files.name);

            WriteTomlContext {
                test_files,
                filename,
            }
        }

        fn teardown(self) {
            self.test_files.teardown();
        }
    }

    use super::*;

    #[test_context(ReadTomlContext)]
    #[test]
    fn test_read_toml_file(ctx: &mut ReadTomlContext) {
        #[derive(Deserialize)]
        struct Data {
            hello: String,

            section_a: SectionA,
        }

        #[derive(Deserialize)]
        struct SectionA {
            abc: String,
            xyz: u32,
        }

        let data = read_toml_file::<Data>(Path::new(&ctx.filename)).unwrap();

        assert_eq!(data.hello, "world");
        assert_eq!(data.section_a.abc, "CBA");
        assert_eq!(data.section_a.xyz, 54626);
    }

    #[test_context(ReadTomlContext)]
    #[test]
    fn test_read_toml_file_with_optionals(ctx: &mut ReadTomlContext) {
        #[derive(Deserialize)]
        struct Data {
            hello: String,
            name: Option<String>,

            section_a: SectionA,
        }

        #[derive(Deserialize)]
        struct SectionA {
            abc: String,
            ghi: Option<bool>,
            xyz: u32,
        }

        let data = read_toml_file::<Data>(Path::new(&ctx.filename)).unwrap();

        assert_eq!(data.hello, "world");
        assert_eq!(data.name, Some(String::from("Santa Claus")));
        assert_eq!(data.section_a.abc, "CBA");
        assert_eq!(data.section_a.xyz, 54626);
        assert_eq!(data.section_a.ghi, None);
    }

    #[test_context(WriteTomlContext)]
    #[test]
    fn test_write_toml_file(ctx: &mut WriteTomlContext) {
        #[derive(Serialize)]
        struct Data {
            hello: String,

            section_a: SectionA,
        }

        #[derive(Serialize)]
        struct SectionA {
            abc: String,
            xyz: u32,
        }

        let data = Data {
            hello: String::from("world"),
            section_a: SectionA {
                abc: String::from("CBA"),
                xyz: 54626,
            },
        };

        let result = write_toml_file(Path::new(&ctx.filename), &data, false);

        assert!(result.is_ok());

        let mut file = fs::File::open(&ctx.filename).unwrap();

        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(
            buffer,
            "hello = \"world\"\n\n[section_a]\nabc = \"CBA\"\nxyz = 54626\n"
        );
    }

    #[test_context(WriteTomlContext)]
    #[test]
    fn test_write_toml_file_with_optionals(ctx: &mut WriteTomlContext) {
        #[derive(Serialize)]
        struct Data {
            hello: String,
            name: Option<String>,

            section_a: SectionA,
        }

        #[derive(Serialize)]
        struct SectionA {
            abc: String,
            ghi: Option<bool>,
            xyz: u32,
        }

        let data = Data {
            hello: String::from("world"),
            name: None,
            section_a: SectionA {
                abc: String::from("CBA"),
                ghi: Some(true),
                xyz: 54626,
            },
        };

        let result = write_toml_file(Path::new(&ctx.filename), &data, false);

        assert!(result.is_ok());

        let mut file = fs::File::open(&ctx.filename).unwrap();

        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(
            buffer,
            "hello = \"world\"\n\n[section_a]\nabc = \"CBA\"\nghi = true\nxyz = 54626\n"
        );
    }
}
