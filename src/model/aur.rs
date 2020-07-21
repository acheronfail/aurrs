use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AurRpcInfoResult {
    #[serde(rename = "ID")]
    pub id: usize,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "PackageBaseID")]
    pub package_base_id: usize,
    #[serde(rename = "PackageBase")]
    pub package_base: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "NumVotes")]
    pub num_votes: usize,
    #[serde(rename = "OutOfDate")]
    pub out_of_date: Option<usize>,
    #[serde(rename = "Maintainer")]
    pub maintainer: String,
    #[serde(rename = "FirstSubmitted")]
    pub first_submitted: usize,
    #[serde(rename = "LastModified")]
    pub last_modified: usize,
    #[serde(rename = "License")]
    pub license: String,
    #[serde(rename = "URLPath")]
    pub url_path: String,
    #[serde(rename = "CategoryID")]
    pub category_id: usize,
}

#[derive(Debug, Deserialize)]
pub struct AurRpcInfo {
    pub version: usize,
    pub r#type: String,
    pub resultcount: usize,
    pub results: AurRpcInfoResult,
}
