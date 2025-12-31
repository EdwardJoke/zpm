use crate::download::{download_file, extract_tarball, extract_zip};
use crate::models::ZlsRelease;
use crate::utils::{file_exists, get_cache_dir, get_current_file, get_local_bin_dir, get_platform_string, get_version_dir};
use reqwest::Client;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use tokio::fs::create_dir_all;

// ZLS repository release URL
const ZLS_RELEASES_URL: &str = "https://api.github.com/repos/zigtools/zls/releases/latest";

// Find the ZLS binary in the extracted directory
fn find_zls_binary(zls_dir: &Path) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Look for the zls binary in the extracted directory
    let mut queue = vec![zls_dir.to_path_buf()];
    
    while let Some(dir) = queue.pop() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    queue.push(path);
                } else if entry.file_name() == "zls" {
                    return Ok(path);
                }
            }
        }
    }
    
    Err("Failed to find ZLS binary in extracted archive".into())
}

// Install ZLS for the current Zig version
pub async fn install_zls(client: &Client, home_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Installing ZLS...");
    
    // Get the current Zig version
    let current_file = get_current_file(home_dir);
    let current_version = if file_exists(&current_file) {
        fs::read_to_string(current_file).map(|s| s.trim().to_string())
            .map_err(|e| format!("Failed to read current version: {}", e))?
    } else {
        return Err("No current Zig version set. Please install a Zig version first.".into());
    };
    
    println!("Installing ZLS for Zig version {}", current_version);
    
    // Get the platform string
    let platform = get_platform_string()?;
    let zls_platform = match platform.as_str() {
        "x86_64-macos" => "x86_64-macos",
        "aarch64-macos" => "aarch64-macos",
        "x86_64-linux" => "x86_64-linux",
        "aarch64-linux" => "aarch64-linux",
        _ => return Err(format!("Platform {} not supported for ZLS", platform).into()),
    };
    
    // Fetch the latest ZLS release
    let response = client.get(ZLS_RELEASES_URL)
        .header("User-Agent", "zpm")
        .send()
        .await?;
    
    let release: ZlsRelease = response.json().await?;
    
    // Find the appropriate asset for the platform
    let zls_asset = release.assets.into_iter()
        .find(|asset| asset.name.contains(zls_platform))
        .ok_or(format!("No ZLS asset found for platform {}", zls_platform))?;
    
    println!("Downloading ZLS from: {}", zls_asset.browser_download_url);
    
    // Download directory structure
    let zls_dir = get_version_dir(home_dir, &current_version).join("zls");
    let cache_dir = get_cache_dir(home_dir);
    let zls_archive = cache_dir.join(zls_asset.name.clone());
    
    // Download the ZLS archive
    download_file(client, &zls_asset.browser_download_url, &zls_archive).await?;
    
    // Create ZLS directory
    create_dir_all(&zls_dir).await?;
    
    // Extract the ZLS archive (handling both zip and tar.xz formats)
    println!("Extracting ZLS to {}...", zls_dir.display());
    
    if zls_asset.name.ends_with(".zip") {
        // Use unzip command
        extract_zip(&zls_archive, &zls_dir)?;
    } else if zls_asset.name.ends_with(".tar.xz") {
        // Use tar command
        extract_tarball(&zls_archive, &zls_dir)?;
    } else {
        return Err(format!("Unknown ZLS archive format: {}", zls_asset.name).into());
    }
    
    // Find the ZLS binary
    let zls_binary = find_zls_binary(&zls_dir)?;
    
    // Create a symlink for ZLS in the same directory as Zig
    let zls_symlink = get_local_bin_dir(home_dir).join("zls");
    
    if file_exists(&zls_symlink) {
        fs::remove_file(&zls_symlink)?;
    }
    
    symlink(zls_binary, zls_symlink)?;
    
    println!("Successfully installed ZLS for Zig version {}", current_version);
    Ok(())
}
