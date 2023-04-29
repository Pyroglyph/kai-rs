# kai-rs
An experimental Rust library for using large language models via KoboldAI

## Installation
```toml
[dependencies]
# ...
kai-rs = { git = "https://github.com/Pyroglyph/kai-rs" }
```

## ⚠️ Important Note ⚠️
Unless you use [my fork of KoboldAI](https://github.com/Pyroglyph/KoboldAI-Client), this library will only be able to load models in CPU-mode. If you wish you can still load models onto your GPU(s) through the official GUI without my fork.

I have sumbitted a [pull request](https://github.com/KoboldAI/KoboldAI-Client/pull/318) to have this fixed in the official repo.

## Usage
Usage should be pretty straightforward. Here is the general idea:
1. Make a `KoboldClient`
2. Load a `Model` and set your GPU layers. Local models must begin with `./` for now. Automatically setting GPU layers is not implemented yet, so just use the value the KoboldAI GUI gives you, or set it to `vec![0]` to use your CPU.
3. Populate `GenerationSettings` or use the reasonable defaults.
4. Pass a prompt and the settings into `generate` and await the results!

Example:
```rust
use kai_rs::api::v1::*;
use kai_rs::prelude::*;

let kai = KoboldClient::new("http://localhost:5000", APIVersion::V1);
kai.load_model(Model::from("PygmalionAI/pygmalion-6b"), vec![28]).await?;

let settings = GenerationSettings::default();

let prompt = "You: Hi. How are you?";
let response = kai.generate(prompt, settings).await?;

// response => [ "Me: I'm good. I just got back from the gym. What about you?" ]
```

## Contributing
Any contributions are welcome, just open an issue or a PR!