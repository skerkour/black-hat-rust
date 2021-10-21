use std::path::PathBuf;

use crate::Error;

pub const AGENT_INSTALL_FILE: &str = "ch13_agent";
pub const SINGLE_INSTANCE_IDENTIFIER: &str = "ch13_agent";
pub const INSTALL_DIRECTORY: &str = "bhr_ch13";

pub fn get_agent_directory() -> Result<PathBuf, Error> {
    let mut data_dir = match dirs::data_dir() {
        Some(home_dir) => home_dir,
        None => return Err(Error::Internal("Error getting data directory.".to_string())),
    };

    data_dir.push(INSTALL_DIRECTORY);

    Ok(data_dir)
}

pub fn get_agent_install_target() -> Result<PathBuf, Error> {
    let mut install_target = get_agent_directory()?;
    install_target.push(AGENT_INSTALL_FILE);

    Ok(install_target)
}
