use crate::Error;
use rand::{thread_rng, Rng};

pub fn run() -> Result<(), Error> {
    let mut token = [0u8; 64];
    thread_rng().fill(&mut token[..]);
    let base64_token = base64::encode(token);
    println!("{}", base64_token);
    Ok(())
}
