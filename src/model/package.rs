#[derive(Debug)]
pub struct PackageStatus {
    pub name: String,
    pub voted: bool,
    pub token: Option<String>,
}
