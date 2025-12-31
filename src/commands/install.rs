use crate::download::{download_file, extract_tarball, verify_checksum};
use crate::models::{ReleaseIndex, VersionEntry};
use crate::utils::{file_exists, get_cache_dir, get_platform_string, get_version_dir, get_zig_binary, version_compare};
use reqwest::Client;
use tokio::fs::create_dir_all;

const INDEX_URL: &str = "https://ziglang.org/download/index.json";

// Fetch the list of available releases
async fn fetch_releases(client: &Client) -> Result<ReleaseIndex, Box<dyn std::error::Error>> {
    let response = client.get(INDEX_URL).send().await?;
    let releases: ReleaseIndex = response.json().await?;
    Ok(releases)
}

// Get the version entry for a specific version
fn get_version_entry<'a>(releases: &'a ReleaseIndex, version: &str) -> Result<(String, &'a VersionEntry), Box<dyn std::error::Error>> {
    // Determine the exact version to install
    let target_version = match version {
        "latest" | "master" => "master".to_string(),
        "stable" => {
            // Find the latest stable version (non-master)
            let mut versions: Vec<String> = releases.versions.keys()
                .filter(|k| *k != "master")
                .cloned()
                .collect();
            versions.sort_by(|a, b| version_compare(a, b));
            versions.first().unwrap_or(&"master".to_string()).clone()
        },
        v => v.to_string(),
    };

    // Get the version entry
    let version_entry = releases.versions.get(&target_version)
        .ok_or(format!("Version {} not found", target_version))?;

    Ok((target_version, version_entry))
}

// Install a Zig version
pub async fn install(
    client: &Client,
    home_dir: &str,
    version: &str,
    set_as_default: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Installing Zig version: {}", version);
    
    let releases = fetch_releases(client).await?;
    let (target_version, version_entry) = get_version_entry(&releases, version)?;
    
    // Get platform string (e.g., x86_64-macos)
    let platform = get_platform_string()?;
    println!("Platform: {}", platform);
    
    // Get platform-specific entry
    let platform_entry = version_entry.platforms.get(&platform)
        .ok_or(format!("Platform {} not supported for version {}", platform, target_version))?;
    
    // Check if version is already installed
    let version_dir = get_version_dir(home_dir, &target_version);
    let zig_binary = get_zig_binary(home_dir, &target_version);
    
    if file_exists(&zig_binary) {
        println!("Zig version {} is already installed", target_version);
        if set_as_default {
            crate::commands::use_cmd::set_default(home_dir, &target_version).await?;
        }
        return Ok(());
    }
    
    // Download and install
    let cache_dir = get_cache_dir(home_dir);
    let archive_filename = format!("zig-{}-{}.tar.xz", platform, target_version);
    let archive_path = cache_dir.join(archive_filename);
    
    // Download the tarball
    println!("Downloading {}...", platform_entry.tarball);
    download_file(client, &platform_entry.tarball, &archive_path).await?;
    
    // Verify checksum
    println!("Verifying checksum...");
    verify_checksum(&archive_path, &platform_entry.shasum)?;
    
    // Extract the tarball
    println!("Extracting to {}...", version_dir.display());
    create_dir_all(&version_dir).await?;
    extract_tarball(&archive_path, &version_dir)?;
    
    // Set as default if requested
    if set_as_default {
        crate::commands::use_cmd::set_default(home_dir, &target_version).await?;
    }
    
    println!("Successfully installed Zig version {}", target_version);
    Ok(())
}
