use crate::core::transcript::base::TranscriptPart;
use anyhow::anyhow;
use ytranscript::{TranscriptConfig, YoutubeTranscript};

pub async fn get_youtube_video_transcript(
    video_id: &str,
    language: Option<String>,
) -> Result<Vec<TranscriptPart>, anyhow::Error> {
    match YoutubeTranscript::fetch_transcript(
        video_id,
        language.map(|l| TranscriptConfig { lang: Some(l) }),
    )
    .await
    {
        Ok(transcripts) => Ok(transcripts.iter().map(|t| t.into()).collect()),
        Err(err) => Err(anyhow!(err)),
    }
}
