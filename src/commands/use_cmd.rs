use crate::utils::{file_exists, get_current_file, get_local_bin_dir, get_zig_binary, get_zig_symlink};
use std::fs;
use std::io::Write;
use std::os::unix::fs::symlink;
use tokio::fs::create_dir_all;

// Set the default Zig version
pub async fn set_default(home_dir: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting default version to: {}", version);
    
    let zig_binary = get_zig_binary(home_dir, version);
    let zig_symlink = get_zig_symlink(home_dir);
    let current_file = get_current_file(home_dir);
    
    if !file_exists(&zig_binary) {
        return Err(format!("Version {} is not installed", version).into());
    }
    
    // Create local bin directory if it doesn't exist
    let local_bin_dir = get_local_bin_dir(home_dir);
    create_dir_all(local_bin_dir).await?;
    
    // Remove existing symlink if it exists
    if file_exists(&zig_symlink) {
        fs::remove_file(&zig_symlink)?;
    }
    
    // Create new symlink
    symlink(&zig_binary, zig_symlink)?;
    
    // Update current file
    let mut file = std::fs::File::create(current_file)?;
    file.write_all(version.as_bytes())?;
    
    println!("Default version set to: {}", version);
    Ok(())
}
