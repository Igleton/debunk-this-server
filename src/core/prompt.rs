pub struct PromptRepository {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(sqlx::FromRow)]
pub struct Prompt {
    pub id: String,
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
            "SELECT prompt FROM prompts WHERE name = ? ORDER BY created_at DESC",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
    }
}
