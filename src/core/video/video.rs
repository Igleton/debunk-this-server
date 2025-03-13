use anyhow::{anyhow, Error};
use rustypipe::client::RustyPipe;
use rustypipe::model::VideoDetails;

pub async fn get_video_info(video_id: &str) -> Result<VideoDetails, Error> {
    let rp = RustyPipe::new();
    match rp.query().video_details(video_id).await {
        Ok(value) => Ok(value),
        Err(error) => Err(anyhow!(error)),
    }
}