//! Structs for describing the content of a Cargo.toml file
//!

/// Used for describing the content of a Cargo.toml file.
pub struct CargoToml<'a> {
    /// Holds an optional [PackageSection] object.
    pub package: Option<PackageSection<'a>>,

    /// Holds an optional list of the crate names
    /// of the member crates that the root crate depends on.
    pub dependencies: Option<&'a Vec<String>>,

    /// Holds an optional [WorkspaceSection] object.
    pub workspace: Option<WorkspaceSection<'a>>,
}

/// Represents the \[workspace\] section of a Cargo.toml
pub struct WorkspaceSection<'a> {
    pub members: &'a Vec<String>,
}


/// Represents the \[package\] section of a Cargo.toml
pub struct PackageSection<'a> {
    pub pkg_name: &'a str,
    pub pkg_version: &'a str,
    pub pkg_edition: &'a str,
}
