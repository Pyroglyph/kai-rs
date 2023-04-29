use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize)]
pub struct GenerationSettings {
    /// When enabled, all input formatting options default to false instead of the value in the KoboldAI GUI
    pub disable_input_formatting: Option<bool>,

    /// When enabled, all output formatting options default to false instead of the value in the KoboldAI GUI.
    pub disable_output_formatting: Option<bool>,

    /// Input formatting option. When enabled, adds a leading space to your input if there is no trailing whitespace at the end of the previous action. If disable_input_formatting is true, this defaults to false instead of the value in the KoboldAI GUI.
    #[serde(rename = "frmtadsnsp")]
    pub add_leading_space: Option<bool>,

    /// Output formatting option. When enabled, replaces all occurrences of two or more consecutive newlines in the output with one newline. If disable_output_formatting is true, this defaults to false instead of the value in the KoboldAI GUI.
    #[serde(rename = "frmtrmblln")]
    pub trim_blank_lines: Option<bool>,

    /// Output formatting option. When enabled, removes #/@%{}+=~|\^<> from the output. If disable_output_formatting is true, this defaults to false instead of the value in the KoboldAI GUI.
    #[serde(rename = "frmtrmspch")]
    pub trim_special_characters: Option<bool>,

    /// Output formatting option. When enabled, removes some characters from the end of the output such that the output doesn't end in the middle of a sentence. If the output is less than one sentence long, does nothing. If disable_output_formatting is true, this defaults to false instead of the value in the KoboldAI GUI.
    #[serde(rename = "frmttriminc")]
    pub trim_incomplete_sentences: Option<bool>,

    /// Maximum number of tokens to send to the model.
    pub max_context_length: i32,

    /// Number of tokens to generate.
    pub max_length: i32,

    /// Number of outputs to generate.
    pub n: i32,

    /// When enabled, Generated output will not be displayed in the console.
    pub quiet: Option<bool>,

    /// Base repetition penalty value.
    pub rep_pen: Option<f32>,

    /// Repetition penalty range.
    pub rep_pen_range: Option<i32>,

    /// Repetition penalty slope.
    pub rep_pen_slope: Option<f32>,

    /// If enabled, the generated text will always be the same as long as you use the same RNG seed, input and settings. If disabled, only the sequence of generated texts that you get when repeatedly generating text will be the same given the same RNG seed, input and settings.
    pub sampler_full_determinism: Option<bool>,

    /// Sampler order to be used. If N is the length of this array, then N must be greater than or equal to 6 and the array must be a permutation of the first N non-negative integers.
    pub sampler_order: Option<Vec<i32>>,

    /// RNG seed to use for sampling. If not specified, the global RNG will be used.
    pub sampler_seed: Option<i32>,

    /// Output formatting option. When enabled, removes everything after the first line of the output, including the newline. If disable_output_formatting is true, this defaults to false instead of the value in the KoboldAI GUI.
    #[serde(rename = "singleline")]
    pub single_line: Option<bool>,

    /// Soft prompt to use when generating. If set to the empty string or any other string containing no non-whitespace characters, uses no soft prompt.
    pub soft_prompt: Option<String>,

    /// Temperature value.
    pub temperature: Option<f32>,

    /// Tail free sampling value.
    pub tfs: Option<f32>,

    /// Top-a sampling value.
    pub top_a: Option<f32>,

    /// Top-k sampling value.
    pub top_k: Option<i32>,

    /// Top-p sampling value.
    pub top_p: Option<f32>,

    /// Typical sampling value.
    pub typical: Option<f32>,

    // TODO
    pub use_memory: bool,
    pub use_story: bool,
    pub use_authors_note: bool,
    pub use_world_info: bool,
    pub use_userscripts: bool,
}

impl Default for GenerationSettings {
    fn default() -> Self {
        GenerationSettings {
            disable_input_formatting: Some(true),
            disable_output_formatting: Some(true),
            add_leading_space: Some(false),
            trim_blank_lines: Some(false),
            trim_special_characters: Some(false),
            trim_incomplete_sentences: Some(false),
            max_context_length: 1024,
            max_length: 80,
            n: 1,
            quiet: Some(true),
            rep_pen: Some(1.1),
            rep_pen_range: Some(1024),
            rep_pen_slope: Some(0.7),
            sampler_full_determinism: Some(false),
            sampler_order: Some(vec![6, 0, 1, 2, 3, 4, 5]),
            sampler_seed: Some(0),
            single_line: Some(false),
            soft_prompt: Some(String::new()),
            temperature: Some(0.5),
            tfs: Some(1.0),
            top_a: Some(0.0),
            top_k: Some(0),
            top_p: Some(0.9),
            typical: Some(1.0),

            // TODO
            use_memory: true,
            use_story: true,
            use_authors_note: true,
            use_world_info: true,
            use_userscripts: true,
        }
    }
}
