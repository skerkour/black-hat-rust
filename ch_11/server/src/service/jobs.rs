use super::Service;
use crate::{entities::Job, Error};
use chrono::Utc;
use common::{
    api::{CreateJob, UpdateJobResult},
    crypto,
};
use ed25519_dalek::Verifier;
use std::convert::TryFrom;
use uuid::Uuid;

impl Service {
    pub async fn get_job_result(&self, job_id: Uuid) -> Result<Option<Job>, Error> {
        let job = self.repo.find_job_by_id(&self.db, job_id).await?;

        match &job.encrypted_result {
            Some(_) => {
                self.repo.delete_job(&self.db, job.id).await?;
                Ok(Some(job))
            }
            None => Ok(None),
        }
    }

    pub async fn get_agent_job(&self, agent_id: Uuid) -> Result<Option<Job>, Error> {
        let mut agent = self.repo.find_agent_by_id(&self.db, agent_id).await?;

        agent.last_seen_at = Utc::now();
        // ignore result as an error is not important
        let _ = self.repo.update_agent(&self.db, &agent).await;

        match self.repo.find_job_for_agent(&self.db, agent_id).await {
            Ok(job) => Ok(Some(job)),
            Err(Error::NotFound(_)) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn update_job_result(&self, input: UpdateJobResult) -> Result<(), Error> {
        let mut job = self.repo.find_job_by_id(&self.db, input.job_id).await?;
        let agent = self.repo.find_agent_by_id(&self.db, job.agent_id).await?;

        // validate input
        if input.encrypted_job_result.len() > super::ENCRYPTED_JOB_RESULT_MAX_SIZE {
            return Err(Error::InvalidArgument("Result is too large".to_string()));
        }

        if input.signature.len() != crypto::ED25519_SIGNATURE_SIZE {
            return Err(Error::InvalidArgument(
                "Signature size is not valid".to_string(),
            ));
        }

        let mut job_result_buffer = input.job_id.as_bytes().to_vec();
        job_result_buffer.append(&mut agent.id.as_bytes().to_vec());
        job_result_buffer.append(&mut input.encrypted_job_result.clone());
        job_result_buffer.append(&mut input.ephemeral_public_key.to_vec());
        job_result_buffer.append(&mut input.nonce.to_vec());

        let signature = ed25519_dalek::Signature::try_from(&input.signature[0..64])?;
        let agent_identity_public_key =
            ed25519_dalek::PublicKey::from_bytes(&agent.identity_public_key)?;

        if agent_identity_public_key
            .verify(&job_result_buffer, &signature)
            .is_err()
        {
            return Err(Error::InvalidArgument("Signature is not valid".to_string()));
        }

        job.encrypted_result = Some(input.encrypted_job_result);
        job.result_ephemeral_public_key = Some(input.ephemeral_public_key.to_vec());
        job.result_nonce = Some(input.nonce.to_vec());
        job.result_signature = Some(input.signature);
        self.repo.update_job(&self.db, &job).await
    }

    pub async fn create_job(&self, input: CreateJob) -> Result<Job, Error> {
        // validate input
        if input.encrypted_job.len() > super::ENCRYPTED_JOB_MAX_SIZE {
            return Err(Error::InvalidArgument("Job is too large".to_string()));
        }

        if input.signature.len() != crypto::ED25519_SIGNATURE_SIZE {
            return Err(Error::InvalidArgument(
                "Signature size is not valid".to_string(),
            ));
        }

        let mut job_buffer = input.id.as_bytes().to_vec();
        job_buffer.append(&mut input.agent_id.as_bytes().to_vec());
        job_buffer.append(&mut input.encrypted_job.clone());
        job_buffer.append(&mut input.ephemeral_public_key.to_vec());
        job_buffer.append(&mut input.nonce.to_vec());

        let signature = ed25519_dalek::Signature::try_from(&input.signature[0..64])?;

        if !self
            .config
            .client_identity_public_key
            .verify(&job_buffer, &signature)
            .is_ok()
        {
            return Err(Error::InvalidArgument("Signature is not valid".to_string()));
        }

        let new_job = Job {
            id: input.id,
            agent_id: input.agent_id,
            encrypted_job: input.encrypted_job,
            ephemeral_public_key: input.ephemeral_public_key.to_vec(),
            nonce: input.nonce.to_vec(),
            signature: input.signature,
            encrypted_result: None,
            result_ephemeral_public_key: None,
            result_nonce: None,
            result_signature: None,
        };

        self.repo.create_job(&self.db, &new_job).await?;

        Ok(new_job)
    }
}
