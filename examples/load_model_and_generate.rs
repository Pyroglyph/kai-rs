use kai_rs::api::v1::*;
use kai_rs::prelude::*;

#[tokio::main]
async fn main() {
    let kai = KoboldClient::new("http://localhost:5000", APIVersion::V1);
    // ! KoboldAI will start to download the model if it doesn't already exist on-disk
    // ! You will also need to modify gpu_layers for your system/model
    kai.load_model(Model::from("PygmalionAI/pygmalion-6b"), vec![28])
        .await
        .unwrap();

    let settings = GenerationSettings::default();

    let prompt = "You: Hi. How are you?";
    let response = kai.generate(prompt, settings).await.unwrap();

    println!("{response:?}");

    // response => [ "Me: I'm good. I just got back from the gym. What about you?" ]
}
