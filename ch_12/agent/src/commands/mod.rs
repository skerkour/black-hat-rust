mod exec;

#[cfg(target_os = "linux")]
mod install_linux;
#[cfg(target_os = "macos")]
mod install_macos;
#[cfg(target_os = "windows")]
mod install_windows;
