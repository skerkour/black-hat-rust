use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct ServerState {
    pub db: Pool<Postgres>,
}

impl ServerState {
    pub fn new(db: Pool<Postgres>) -> ServerState {
        ServerState { db }
    }
}
