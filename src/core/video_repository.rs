use chrono::Utc;
use rustypipe::model::VideoDetails;
use rustypipe::model::richtext::ToMarkdown;
use rustypipe::model::traits::YtEntity;
use sqlx::Row;

pub struct VideoRepository {
    pub pool: sqlx::PgPool,
}

#[derive(sqlx::FromRow)]
pub struct VideoInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub channel: String,
    pub channel_name: String
}

impl Into<VideoInfo> for VideoDetails {
    fn into(self) -> VideoInfo {
        VideoInfo {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            description: self.description.to_markdown(),
            channel: self.channel_id().clone().unwrap_or("").to_string(),
            channel_name: self.channel_name().unwrap_or("").to_string(),
        }
    }
}

impl VideoRepository {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, video: VideoInfo) -> Result<(), sqlx::Error> {
        let video_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM video WHERE name = $1)")
                .bind(video.name.to_owned())
                .fetch_one(&self.pool)
                .await?;
        if video_exists {
            return Ok(());
        }
        sqlx::query("INSERT INTO video (id, name, description, channel) VALUES ($1, $2, $3, $4)")
            .bind(video.id)
            .bind(video.name)
            .bind(video.description)
            .bind(video.channel)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get(&self, video_id: String) -> Result<Option<VideoInfo>, sqlx::Error> {
        Ok(
            sqlx::query_as::<_, VideoInfo>("SELECT * FROM video WHERE id = $1")
                .bind(video_id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }
}
