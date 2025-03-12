use serde::Serialize;

#[derive(Serialize)]
pub struct TranscriptPart {
    pub text: String,
    pub duration: f64,
    pub offset: f64,
    pub lang: String,
}

impl From<&ytranscript::types::TranscriptResponse> for TranscriptPart {
    fn from(resp: &ytranscript::types::TranscriptResponse) -> Self {
        TranscriptPart {
            text: resp.text.to_owned(),
            duration: resp.duration,
            offset: resp.offset,
            lang: resp.lang.to_owned(),
        }
    }
}
