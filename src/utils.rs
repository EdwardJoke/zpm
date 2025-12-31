use std::path::{Path, PathBuf};

const ZPM_DIR: &str = ".zpm";
const VERSIONS_DIR: &str = "versions";
const CACHE_DIR: &str = "cache";
const CURRENT_FILE: &str = "current";

// File system utilities
pub fn file_exists(path: &Path) -> bool {
    path.exists()
}

// Path management functions
pub fn get_zpm_dir(home_dir: &str) -> PathBuf {
    Path::new(home_dir).join(ZPM_DIR)
}

pub fn get_versions_dir(home_dir: &str) -> PathBuf {
    get_zpm_dir(home_dir).join(VERSIONS_DIR)
}

pub fn get_cache_dir(home_dir: &str) -> PathBuf {
    get_zpm_dir(home_dir).join(CACHE_DIR)
}

pub fn get_current_file(home_dir: &str) -> PathBuf {
    get_zpm_dir(home_dir).join(CURRENT_FILE)
}

pub fn get_version_dir(home_dir: &str, version: &str) -> PathBuf {
    get_versions_dir(home_dir).join(version)
}

pub fn get_zig_binary(home_dir: &str, version: &str) -> PathBuf {
    get_version_dir(home_dir, version).join("zig")
}

pub fn get_local_bin_dir(home_dir: &str) -> PathBuf {
    Path::new(home_dir).join(".local").join("bin")
}

pub fn get_zig_symlink(home_dir: &str) -> PathBuf {
    get_local_bin_dir(home_dir).join("zig")
}

// Platform detection
pub fn get_platform_string() -> Result<String, Box<dyn std::error::Error>> {
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        "arm" => "arm",
        _ => return Err(format!("Unsupported architecture: {}", std::env::consts::ARCH).into()),
    };
    
    let os = match std::env::consts::OS {
        "macos" => "macos",
        "linux" => "linux",
        "windows" => "windows",
        _ => return Err(format!("Unsupported OS: {}", std::env::consts::OS).into()),
    };
    
    Ok(format!("{}-{}", arch, os))
}

// Version comparison
pub fn version_compare(a: &str, b: &str) -> std::cmp::Ordering {
    let a_is_master = a == "master";
    let b_is_master = b == "master";
    
    if a_is_master && b_is_master {
        return std::cmp::Ordering::Equal;
    }
    if a_is_master {
        return std::cmp::Ordering::Greater;
    }
    if b_is_master {
        return std::cmp::Ordering::Less;
    }
    
    let a_parts: Vec<&str> = a.split('.').collect();
    let b_parts: Vec<&str> = b.split('.').collect();
    
    for (a_part, b_part) in a_parts.iter().zip(b_parts.iter()) {
        let a_num = a_part.parse::<u32>().unwrap_or(0);
        let b_num = b_part.parse::<u32>().unwrap_or(0);
        
        if a_num != b_num {
            return b_num.cmp(&a_num);
        }
    }
    
    b_parts.len().cmp(&a_parts.len())
}
