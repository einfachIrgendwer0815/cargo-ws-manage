use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug)]
pub enum IOError {
    InvalidPath,
    TomlSerError(toml::ser::Error),
    TomlDeError(toml::de::Error),
    FsError(io::Error),
}

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

pub fn read_toml_file<T: for<'a> Deserialize<'a>>(path: &Path) -> Result<T, IOError> {
    let data = read_file(path)?;
    match toml::from_str::<T>(&data) {
        Ok(d) => Ok(d),
        Err(e) => {
            return Err(IOError::TomlDeError(e));
        }
    }
}

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

fn unpack_path(path: &Path) -> Result<&str, IOError> {
    match path.to_str() {
        Some(s) => Ok(s),
        None => {
            return Err(IOError::InvalidPath);
        }
    }
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
