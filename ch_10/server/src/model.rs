pub struct Agent {
    id: uuid::Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    last_seen_at: chrono::DateTime<chrono::Utc>,
}