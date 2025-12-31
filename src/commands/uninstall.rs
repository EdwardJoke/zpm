use crate::utils::{file_exists, get_current_file, get_version_dir, get_zig_binary, get_zig_symlink};
use std::fs;

// Uninstall a Zig version
pub async fn uninstall(home_dir: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Uninstalling Zig version: {}", version);
    
    let version_dir = get_version_dir(home_dir, version);
    let zig_binary = get_zig_binary(home_dir, version);
    
    if !file_exists(&zig_binary) {
        return Err(format!("Version {} is not installed", version).into());
    }
    
    // Check if this is the current default version
    let current_file_path = get_current_file(home_dir);
    let zig_symlink = get_zig_symlink(home_dir);
    
    let is_current = if file_exists(&current_file_path) {
        let current = fs::read_to_string(&current_file_path).ok();
        current.map(|s| s.trim() == version).unwrap_or(false)
    } else {
        false
    };
    
    // Remove the version directory
    fs::remove_dir_all(version_dir)?;
    
    // If this was the current version, remove the symlink and current file
    if is_current {
        if file_exists(&zig_symlink) {
            fs::remove_file(zig_symlink)?;
        }
        if file_exists(&current_file_path) {
            fs::remove_file(current_file_path)?;
        }
        println!("Removed default version {}", version);
    }
    
    println!("Successfully uninstalled Zig version {}", version);
    Ok(())
}
