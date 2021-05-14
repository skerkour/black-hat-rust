mod consts;
mod error;
mod init;
mod run;

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent_id = init::init()?;
    run::run(agent_id);
}
