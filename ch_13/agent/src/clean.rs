use crate::config;
use std::fs;

pub fn clean() -> Result<(), crate::Error> {
    let install_dir = config::get_agent_directory()?;
    fs::remove_dir_all(install_dir)?;
    Ok(())
}
