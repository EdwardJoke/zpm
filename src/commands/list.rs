use crate::models::ReleaseIndex;
use crate::utils::{file_exists, get_current_file, get_versions_dir, version_compare};
use reqwest::Client;
use std::collections::HashSet;
use std::fs;

const INDEX_URL: &str = "https://ziglang.org/download/index.json";

// Get installed versions and current version
async fn get_installed_versions(home_dir: &str) -> Result<(Vec<String>, Option<String>), Box<dyn std::error::Error>> {
    let versions_dir = get_versions_dir(home_dir);
    let current_file = get_current_file(home_dir);
    
    let current_version = if file_exists(&current_file) {
        fs::read_to_string(current_file).ok().map(|s| s.trim().to_string())
    } else {
        None
    };
    
    let mut versions: Vec<String> = Vec::new();
    
    if let Ok(dir) = fs::read_dir(versions_dir) {
        for entry in dir.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                versions.push(entry.file_name().to_string_lossy().to_string());
            }
        }
    }
    
    Ok((versions, current_version))
}

// Fetch the list of available releases
async fn fetch_releases(client: &Client) -> Result<ReleaseIndex, Box<dyn std::error::Error>> {
    let response = client.get(INDEX_URL).send().await?;
    let releases: ReleaseIndex = response.json().await?;
    Ok(releases)
}

// List all available Zig versions with tags
pub async fn list_versions(client: &Client, home_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Listing all available Zig versions:");
    println!("  * = current version, I = installed");
    println!();
    
    // Get available versions
    let releases = fetch_releases(client).await?;
    
    // Get installed versions and current version
    let (installed, current_version) = get_installed_versions(home_dir).await?;
    let installed_set: HashSet<_> = installed.into_iter().collect();
    
    // Prepare and sort all versions
    let mut all_versions: Vec<String> = releases.versions.keys().cloned().collect();
    all_versions.sort_by(|a, b| version_compare(a, b));
    
    // Display versions with tags
    for version in all_versions {
        let is_installed = installed_set.contains(&version);
        let is_current = current_version.as_ref().map(|v| v == &version).unwrap_or(false);
        
        let mut markers = String::new();
        if is_current {
            markers.push('*');
        } else {
            markers.push(' ');
        }
        
        if is_installed {
            markers.push('I');
        } else {
            markers.push(' ');
        }
        
        println!("  {} {}", markers, version);
    }
    
    Ok(())
}
