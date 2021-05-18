use super::Service;
use crate::{entities::Job, Error};
use chrono::Utc;
use common::{
    api::{CreateJob, UpdateJobResult},
    crypto,
};
use ed25519_dalek::Verifier;
use uuid::Uuid;

impl Service {
    pub async fn get_job_result(&self, job_id: Uuid) -> Result<Option<Job>, Error> {
        let job = self.repo.find_job_by_id(&self.db, job_id).await?;

        match &job.encrypted_result {
            Some(_) => Ok(Some(job)),
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

        job.executed_at = Some(Utc::now());
        job.output = Some(input.output);
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

        let mut job_buffer_iter = input.id.as_bytes().into_iter();
        job_buffer_iter.chain(input.agent_id.as_bytes());
        job_buffer_iter.chain(input.encrypted_job.clone());
        job_buffer_iter.chain(input.ephemeral_public_key.clone());
        job_buffer_iter.chain(input.nonce.clone());

        let job_buffer: Vec<u8> = job_buffer_iter.collect();

        if !self
            .config
            .client_identity_public_key
            .verify(&job_buffer, &input.signature)
            .is_ok()
        {
            return Err(Error::InvalidArgument("Signature is not valid".to_string()));
        }

        let now = Utc::now();
        let new_job = Job {
            id: input.id,
            agent_id: input.agent_id,
            encrypted_job: input.encrypted_job,
            ephemeral_public_key: input.ephemeral_public_key,
            nonce: input.nonce,
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
