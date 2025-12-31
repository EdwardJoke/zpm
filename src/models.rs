use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlatformEntry {
    pub tarball: String,
    pub shasum: String,
}

#[derive(Deserialize, Debug)]
pub struct VersionEntry {
    #[serde(default)]
    pub platforms: std::collections::HashMap<String, PlatformEntry>,
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
