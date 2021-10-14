use crate::config;
use std::path::PathBuf;
use std::{env, fs, io};

pub fn install() -> Result<PathBuf, crate::Error> {
    let install_dir = config::get_agent_directory()?;
    let install_target = config::get_agent_install_target()?;

    if !install_target.exists() {
        println!("Installing into {}", install_dir.display());
        let current_exe = env::current_exe()?;

        fs::create_dir_all(&install_dir)?;

        fs::copy(current_exe, &install_target)?;

        // copy and extract bundle.zip
        let bundle = PathBuf::from("bundle.zip");
        if bundle.exists() {
            println!(
                "bundle.zip found, extracting it to {}",
                install_dir.display()
            );

            let mut dist_bundle = install_dir.clone();
            dist_bundle.push(&bundle);

            fs::copy(&bundle, &dist_bundle)?;

            let zip_file = fs::File::open(&dist_bundle)?;
            let mut zip_archive = zip::ZipArchive::new(zip_file)?;

            for i in 0..zip_archive.len() {
                let mut archive_file = zip_archive.by_index(i)?;
                let dist_filename = match archive_file.enclosed_name() {
                    Some(path) => path.to_owned(),
                    None => continue,
                };
                let mut dist_path = install_dir.clone();
                dist_path.push(dist_filename);

                let mut dist_file = fs::File::create(&dist_path)?;
                io::copy(&mut archive_file, &mut dist_file)?;
            }
        } else {
            println!("bundle.zip NOT found");
        }
    }

    Ok(install_dir)
}
