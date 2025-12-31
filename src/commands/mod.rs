// Command modules
pub mod install;
pub mod install_zls;
pub mod list;
pub mod uninstall;
pub mod use_cmd;

// Re-export command functions
pub use install::install;
pub use install_zls::install_zls;
pub use list::list_versions;
pub use uninstall::uninstall;
pub use use_cmd::set_default;
