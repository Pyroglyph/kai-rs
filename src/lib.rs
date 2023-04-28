mod generation_response;
mod generation_settings;
mod model;
mod string_response;

use anyhow::{bail, Result};
use generation_response::{GenerationErrorResult, GenerationOkResult};
use generation_settings::GenerationSettings;
use model::Model;
use serde_json::{json, Value};
use std::fmt;

pub enum APIVersion {
    V1,
}

impl fmt::Display for APIVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIVersion::V1 => write!(f, "v1"),
        }
    }
}

pub struct KoboldClient {
    api_url: String,
    client: reqwest::Client,
}

impl KoboldClient {
    pub fn new(api_url: &str, api_version: APIVersion) -> Self {
        let api_url = format!("{api_url}/api/{api_version}");
        let client = reqwest::Client::new();

        KoboldClient { api_url, client }
    }

    fn get_result_string(value: serde_json::Value) -> Result<String> {
        let string = value
            .get("result")
            .expect("Property 'result' not found.")
            .as_str()
            .expect("Could not convert 'result' into a string.");

        Ok(String::from(string))
    }

    pub async fn get_version(&self) -> Result<String> {
        let response = self
            .client
            .get(format!("{}/info/version", self.api_url))
            .send()
            .await?;

        let version = Self::get_result_string(response.json().await?)?;

        Ok(version)
    }

    pub async fn get_model(&self) -> Result<Option<Model>> {
        let response = self
            .client
            .get(format!("{}/model", self.api_url))
            .send()
            .await?;

        let model_string = Self::get_result_string(response.json().await?)?;

        if model_string.eq("ReadOnly") {
            Ok(None)
        } else {
            Ok(Some(Model::from(model_string)))
        }
    }

    pub async fn load_model(&self, model: Model, gpu_layers: Vec<i32>) -> Result<()> {
        let gpu_layers_breakmodel: String = gpu_layers
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let response = self
            .client
            .put(format!("{}/model", self.api_url))
            .json(&json!({ "model": model, "gpu_layers": gpu_layers_breakmodel }))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            bail!(response.status())
        }
    }

    pub async fn generate(
        &self,
        prompt: &str,
        settings: GenerationSettings,
    ) -> Result<Vec<String>> {
        let settings_value = inject_prompt(prompt, settings)?;

        let response = self
            .client
            .post(format!("{}/generate", self.api_url))
            .json(&settings_value)
            .send()
            .await?;

        if response.status().is_success() {
            let generation_response = response.json::<GenerationOkResult>().await?;

            let generations = generation_response
                .results
                .into_iter()
                .map(|generation| generation.text)
                .collect();

            Ok(generations)
        } else {
            println!("HTTP {}", response.status());

            let error_response = &response.json::<GenerationErrorResult>().await?;

            bail!(format!(
                "[{:?}] {}",
                &error_response.detail.error_type, &error_response.detail.message
            ));
        }
    }
}

fn inject_prompt(prompt: &str, settings: GenerationSettings) -> Result<Value> {
    let mut settings_value = serde_json::to_value(&settings)?;

    // inject prompt into settings object before serializing
    settings_value = match settings_value {
        Value::Object(m) => {
            let mut m = m.clone();
            m.insert(
                "prompt".into(),
                serde_json::Value::String(prompt.to_string()),
            );

            Value::Object(m)
        }
        _ => bail!("GenerationSettings object was not an object!"),
    };

    Ok(settings_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ! Tests prefixed with "debug" are not unit tests. You need KoboldAI running to use them.

    #[tokio::test]
    async fn debug_get_version() {
        let kai = KoboldClient::new("http://localhost:5000", APIVersion::V1);
        kai.get_version().await.unwrap();
    }

    #[tokio::test]
    async fn debug_get_model() {
        let kai = KoboldClient::new("http://localhost:5000", APIVersion::V1);
        let model = kai.get_model().await.unwrap();
        dbg!(model);
    }

    #[tokio::test]
    async fn debug_load_model() {
        let kai = KoboldClient::new("http://localhost:5000", APIVersion::V1);
        // ! System dependent
        let gpu_layers = vec![28];
        kai.load_model(Model::from("./pygmalion-6b_dev"), gpu_layers)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn debug_generate() {
        let kai = KoboldClient::new("http://localhost:5000", APIVersion::V1);
        let settings = GenerationSettings::default();

        let prompt = "You: Hi. How are you?";
        let response = kai.generate(prompt, settings).await;

        match response {
            Ok(generations) => {
                println!("{}{}", prompt, generations[0]);
                assert!(true)
            }
            Err(err) => {
                dbg!(err);
                assert!(false)
            }
        };
    }

    #[test]
    fn inject_prompt_into_generation_settings_object() {
        let settings = GenerationSettings::default();
        let result = inject_prompt("test".into(), settings).unwrap();

        assert_eq!(
            result
                .as_object()
                .unwrap()
                .get("prompt")
                .unwrap()
                .as_str()
                .unwrap(),
            "test"
        )
    }

    #[test]
    fn decode_successful_generation() {
        let value = json!({
            "results": [{
                "text": "testing"
            }]
        });

        let ok_result = serde_json::from_value::<GenerationOkResult>(value).unwrap();

        assert_eq!(ok_result.results[0].text, "testing")
    }

    #[test]
    fn decode_failed_generation() {
        let value = json!({
            "detail": {
                "msg": "test",
                "type": "not_implemented"
            }
        });

        let err_result = serde_json::from_value::<GenerationErrorResult>(value).unwrap();

        assert_eq!(err_result.detail.message, "test")
    }
}
