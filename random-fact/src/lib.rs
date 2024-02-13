use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::{http_component, llm};

/// A simple Spin HTTP component.
#[http_component]
fn handle_random_fact(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let model = llm::InferencingModel::Llama2Chat;
    let prompt = format!(
        "Give me a random fact for {}",
        String::from_utf8(req.body().to_vec()).unwrap()
    );
    println!("Prompt: {:?}", prompt);
    let inference = llm::infer(model, &prompt);
    if let Ok(result) = &inference {
        println!("Inference result: {:?}", result);
        let text = &result.text;
        return Ok(Response::new(200, text.to_string()));
    } else {
        return Ok(Response::new(500, "Inference failed".to_string()));
    }
}
