pub struct CargoToml<'a> {
    pub package: Option<PackageSection<'a>>,
    pub dependencies: Option<&'a Vec<String>>,
    pub workspace: Option<WorkspaceSection<'a>>,
}

pub struct WorkspaceSection<'a> {
    pub members: &'a Vec<String>,
}

pub struct PackageSection<'a> {
    pub pkg_name: &'a String,
    pub pkg_version: &'a str,
    pub pkg_edition: &'a str,
}
