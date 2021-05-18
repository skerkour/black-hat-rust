use crate::{
    api::Client,
    config::{self, Config},
    Error,
};
use common::{api, crypto};
use std::{thread::sleep, time::Duration};
use uuid::Uuid;

pub fn run(api_client: &Client, agent_id: &str, command: &str, conf: Config) -> Result<(), Error> {
    let agent_id = Uuid::parse_str(agent_id)?;
    let sleep_for = Duration::from_millis(500);

    let mut command_with_args: Vec<String> = command
        .split_whitespace()
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

    if command_with_args.is_empty() {
        return Err(Error::Internal("Command is not valid".to_string()));
    }

    let command = command_with_args.remove(0);
    let args = command_with_args;

    // get agent's info
    let agent = api_client.get_agent(agent_id)?;
    let agent_identity_public_key =
        ed25519_dalek::PublicKey::from_bytes(&agent.identity_public_key)?;

    // encrypt job
    let (input, job_ephemeral_private_key) = encrypt_and_sign_job_result(
        &conf,
        command,
        args,
        agent.public_prekey,
        &agent.public_prekey_signature,
    )?;

    // create job
    let job_id = api_client.create_job(input)?;

    loop {
        let job = api_client.get_job_result(job_id)?;
        if let Some(_) = &job.encrypted_result {
            // decrypt job's output
            let job_output = decrypt_and_verify_job_output(
                &conf,
                job,
                job_ephemeral_private_key,
                &agent_identity_public_key,
            )?;
            println!("{}", job_output);
            break;
        }
        sleep(sleep_for);
    }

    Ok(())
}

fn encrypt_and_sign_job_result(
    conf: &config::Config,
    command: String,
    args: Vec<String>,
    public_prekey: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
    public_prekey_signature: &[u8],
) -> Result<(api::CreateJob, [u8; crypto::X25519_PRIVATE_KEY_SIZE]), Error> {
    if public_prekey_signature.len() != crypto::ED25519_SIGNATURE_SIZE {
        return Err(Error::Internal(
            "Agent's prekey signature size is not valid".to_string(),
        ));
    }

    // generate ephemeral keypair for job result
    // generate ephemeral keypair for job encryption
    // encrypt job

    unimplemented!();
}

fn decrypt_and_verify_job_output(
    conf: &config::Config,
    job: api::Job,
    job_ephemeral_private_key: [u8; crypto::X25519_PRIVATE_KEY_SIZE],
    agent_identity_public_key: &ed25519_dalek::PublicKey,
) -> Result<String, Error> {
    unimplemented!();
}
