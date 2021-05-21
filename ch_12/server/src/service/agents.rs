use super::Service;
use crate::{
    entities::{self, Agent},
    Error,
};
use chrono::Utc;
use common::{api, crypto};
use ed25519_dalek::Verifier;
use std::convert::TryFrom;
use uuid::Uuid;

impl Service {
    pub async fn list_agents(&self) -> Result<Vec<entities::Agent>, Error> {
        self.repo.find_all_agents(&self.db).await
    }

    pub async fn find_agent(&self, agent_id: Uuid) -> Result<entities::Agent, Error> {
        self.repo.find_agent_by_id(&self.db, agent_id).await
    }

    pub async fn register_agent(
        &self,
        input: api::RegisterAgent,
    ) -> Result<api::AgentRegistered, Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();

        // verify input
        if input.public_prekey_signature.len() != crypto::ED25519_SIGNATURE_SIZE {
            return Err(Error::InvalidArgument(
                "Agent's public prekey Signature size is not valid".to_string(),
            ));
        }

        let agent_identity_public_key =
            ed25519_dalek::PublicKey::from_bytes(&input.identity_public_key)?;
        let signature = ed25519_dalek::Signature::try_from(&input.public_prekey_signature[0..64])?;

        log::debug!("register_agent: input is valid");

        if agent_identity_public_key
            .verify(&input.public_prekey, &signature)
            .is_err()
        {
            return Err(Error::InvalidArgument("Signature is not valid".to_string()));
        }

        log::debug!("register_agent: agent's public_prekey signature verified");

        let agent = Agent {
            id,
            created_at,
            last_seen_at: created_at,
            identity_public_key: input.identity_public_key.to_vec(),
            public_prekey: input.public_prekey.to_vec(),
            public_prekey_signature: input.public_prekey_signature.to_vec(),
        };

        self.repo.create_agent(&self.db, &agent).await?;

        Ok(api::AgentRegistered { id })
    }
}
