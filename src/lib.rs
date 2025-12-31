// Main library file defining the module structure

pub mod commands;
pub mod download;
pub mod models;
pub mod utils;

// Re-export common types and functions for easier access
pub use models::{PlatformEntry, ReleaseIndex, VersionEntry};
pub use utils::{file_exists, get_cache_dir, get_current_file, get_local_bin_dir, get_platform_string, get_version_dir, get_versions_dir, get_zig_binary, get_zig_symlink, get_zpm_dir, version_compare};
