use super::Service;
use crate::{
    entities::{self, Agent},
    Error,
};
use chrono::Utc;
use common::api;
use uuid::Uuid;

impl Service {
    pub async fn list_agents(&self) -> Result<Vec<entities::Agent>, Error> {
        self.repo.find_all_agents(&self.db).await
    }

    pub async fn register_agent(
        &self,
        input: api::RegisterAgent,
    ) -> Result<api::AgentRegistered, Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        // verify input
        // if input.signature.len() != crypto::ED25519_SIGNATURE_SIZE {
        //     return Err(Error::InvalidArgument(
        //         "Signature size is not valid".to_string(),
        //     ));
        // }

        // let mut job_result_buffer = input.job_id.as_bytes().to_vec();
        // job_result_buffer.append(&mut input.encrypted_job_result.clone());
        // job_result_buffer.append(&mut input.ephemeral_public_key.to_vec());
        // job_result_buffer.append(&mut input.nonce.to_vec());

        // let signature = ed25519_dalek::Signature::try_from(&input.signature[0..64])?;

        // if !self
        //     .config
        //     .client_identity_public_key
        //     .verify(&job_result_buffer, &signature)
        //     .is_ok()
        // {
        //     return Err(Error::InvalidArgument("Signature is not valid".to_string()));
        // }

        let agent = Agent {
            id,
            created_at,
            last_seen_at: created_at,
            identity_public_key: (),
            public_prekey: (),
            public_prekey_signature: (),
        };

        self.repo.create_agent(&self.db, &agent).await?;

        Ok(AgentRegistered { id })
    }
}
