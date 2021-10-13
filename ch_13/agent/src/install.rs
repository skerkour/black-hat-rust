use crate::config;
use std::path::PathBuf;
use std::{env, fs};

pub fn install() -> Result<PathBuf, crate::Error> {
    let install_target = config::get_agent_install_target()?;

    if !install_target.exists() {
        let current_exe = env::current_exe()?;

        let install_dir = config::get_agent_directory()?;
        fs::create_dir_all(&install_dir)?;

        fs::copy(current_exe, &install_target)?;
    }

    Ok(install_target)
}
