pub mod exec;

#[cfg(target_os = "linux")]
pub mod install_linux;
#[cfg(target_os = "macos")]
pub mod install_macos;
#[cfg(target_os = "windows")]
pub mod install_windows;
