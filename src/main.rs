use axum::{extract::State, routing::get, Router};
use std::sync::Arc;

pub struct WordState {
    pub word: String,
}

impl Default for WordState {
    fn default() -> Self {
        Self {
            word: format!("{}-{}", random_word::gen(), random_word::gen()),
        }
    }
}

#[tokio::main]
async fn main() {
    let state = WordState::default();
    println!("Serve word \"{}\" on 0.0.0.0:8080", state.word);
    let shared_state = Arc::new(state);
    let app = Router::new().route("/", get(root)).with_state(shared_state);
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root(State(state): State<Arc<WordState>>) -> String {
    println!("Responded: {}", state.word);
    format!("{}\n", state.word)
}
