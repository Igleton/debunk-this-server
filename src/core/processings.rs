use crate::core::prompt::PromptRepository;
use sqlx::types::Uuid;
pub struct ProcessingRepository {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ProcessingRepository {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, video_id: &str) -> Result<String, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO _processings(id, video_id, status) VALUES ($1, $2, $3)")
            .bind(id)
            .bind(video_id)
            .bind("STARTING")
            .fetch_one(&self.pool)
            .await?;
        Ok(id.to_string())
    }
}
