CREATE TABLE agents (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  last_seen_at TIMESTAMP WITH TIME ZONE NOT NULL,
  identity_public_key BYTEA NOT NULL,
  public_prekey BYTEA NOT NULL,
  public_prekey_signature BYTEA NOT NULL
);


CREATE TABLE jobs (
  id UUID PRIMARY KEY,

  encrypted_job BYTEA NOT NULL,
  ephemeral_public_key BYTEA NOT NULL,
  nonce BYTEA NOT NULL,
  signature BYTEA NOT NULL,

  encrypted_result BYTEA,
  result_ephemeral_public_key BYTEA,
  result_nonce BYTEA,
  result_signature BYTEA,

  agent_id UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE
);
CREATE INDEX index_jobs_on_agent_id ON jobs (agent_id);
