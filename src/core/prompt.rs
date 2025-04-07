use chrono::Utc;
use sqlx::types::Uuid;

pub struct PromptRepository {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(sqlx::FromRow)]
pub struct Prompt {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub prompt: String,
}

impl PromptRepository {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_prompt(&self, name: String) -> Result<Option<Prompt>, sqlx::Error> {
        sqlx::query_as::<_, Prompt>(
            "SELECT * FROM prompts WHERE name = $1 ORDER BY created_at DESC",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create(&self, name: &str, prompt: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO prompts (name, created_at, prompt) VALUES ($1, $2, $3)")
            .bind(name)
            .bind(Utc::now().naive_utc())
            .bind(prompt)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
