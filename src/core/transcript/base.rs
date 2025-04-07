use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct TranscriptPart {
    pub text: String,
    pub duration: f64,
    pub offset: f64,
    pub lang: String,
}

impl TranscriptPart {
    pub fn to_postgres_entry(&self) -> String {
        format!("ROW('{}', '{}', '{}', '{}')", self.text, self.duration, self.offset, self.lang)
    }
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
