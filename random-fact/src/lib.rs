use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::{http_component, llm};

const DEFAULT_SYSTEM_PROMPT : &str = "I want you to act as a fact generator. I will give you a word or phrase, and you will provide one random, interesting fact about it. Start your response with 'Did you know...' and then provide the fact. The facts should be educational and engaging. Keep each fact concise. Facts should be true and verifiable. Facts should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content.";

/// A simple Spin HTTP component.
#[http_component]
fn handle_random_fact(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let model = llm::InferencingModel::Llama2Chat;
    let phrase = String::from_utf8(req.body().to_vec()).unwrap();
    let prompt = format!(
        "<s>[INST] <<SYS>>\n{}\n<</SYS>>\n\n{} [/INST]",
        DEFAULT_SYSTEM_PROMPT, phrase
    );
    println!("Prompt: {:?}", prompt);
    let inference = llm::infer(model, &prompt);
    if let Ok(result) = &inference {
        println!("Inference result: {:?}", result.text);
        let text = &result.text;
        return Ok(Response::new(200, text.to_string()));
    } else {
        return Ok(Response::new(500, "Inference failed".to_string()));
    }
}
