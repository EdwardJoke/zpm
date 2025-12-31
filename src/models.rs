use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlatformEntry {
    pub tarball: String,
    pub shasum: String,
}

#[derive(Deserialize, Debug)]
pub struct VersionEntry {
    pub version: Option<String>,
    pub date: Option<String>,
    pub docs: Option<String>,
    #[serde(rename = "stdDocs")]
    pub std_docs: Option<String>,
    #[serde(flatten)]
    pub other_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseIndex {
    #[serde(flatten)]
    pub versions: std::collections::HashMap<String, VersionEntry>,
}

// ZLS Release models
#[derive(Deserialize, Debug)]
pub struct ZlsAsset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Deserialize, Debug)]
pub struct ZlsRelease {
    pub assets: Vec<ZlsAsset>,
}
