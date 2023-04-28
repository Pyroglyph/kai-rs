use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GenerationOkResult {
    pub results: Vec<GeneratedText>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GeneratedText {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GenerationErrorResult {
    pub detail: GenerationError,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GenerationError {
    #[serde(rename = "type")]
    pub error_type: GenerationErrorType,
    #[serde(rename = "msg")]
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub(crate) enum GenerationErrorType {
    #[serde(rename = "not_implemented")]
    NotImplemented,
    #[serde(rename = "service_unavailable")]
    ServiceUnavailable,
    #[serde(rename(
        deserialize = "out_of_memory.cpu.default_cpu_allocator",
        deserialize = "out_of_memory.gpu.cuda",
        deserialize = "out_of_memory.gpu.hip",
        deserialize = "out_of_memory.tpu.hbm",
        deserialize = "out_of_memory.unknown.unknown",
    ))]
    OutOfMemory,
}
