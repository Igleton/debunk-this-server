use chrono::Utc;
use crate::core::transcript::base::TranscriptPart;
use sqlx::types::Uuid;

pub struct VideoSynthesisRepository {
    pool: sqlx::PgPool,
}

pub struct VideoSynthesis {
    pub id: String,
    pub video_id: Uuid,
    pub transcript: Vec<TranscriptPart>,
    pub synthesis: String,
}

impl VideoSynthesisRepository {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }
    pub async fn add_synthesis_for_video(
        &self,
        video_id: &str,
        transcript: Vec<TranscriptPart>,
        synthesis: &str,
    ) -> Result<(), anyhow::Error> {
        sqlx::query("INSERT INTO video_synthesis(created_date, video_id, transcript, synthesis) VALUES ($1, $2, ARRAY[$3], $4)")
            .bind(Utc::now().naive_utc())
            .bind(Uuid::parse_str(video_id)?)
            .bind(transcript.iter().map(|t|t.to_postgres_entry()).collect::<Vec<String>>().join(","))
            .bind(synthesis)
            .execute(&self.pool).await?;
        Ok(())
    }
}
