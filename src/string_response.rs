use serde::Deserialize;

#[derive(Deserialize)]
pub struct StringResponse {
    pub result: String,
}
