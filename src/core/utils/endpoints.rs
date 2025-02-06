use std::sync::{Arc, Mutex};

use crate::core::utils::persistence::PersistInMemory;

use super::handlers::{
    add_body, add_headshot, add_paragraph, add_synopsis, create_story, report_story, root,
};
use anyhow::Error;
use axum::{
    routing::{get, post},
    Router,
};

pub async fn init_server() -> Result<(), Error> {
    let shared_state = Arc::new(Mutex::new(PersistInMemory::new()));

    let router_main = init_routes(shared_state);
    let bind_uri = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(&bind_uri).await.unwrap();
    println!("Listening on: {}", &bind_uri);
    axum::serve(listener, router_main).await.unwrap();

    Ok(())
}

fn init_routes(state: Arc<Mutex<PersistInMemory>>) -> Router {
    let router = Router::new()
        .route("/", get(root))
        .route("/api/v0/story", post(create_story))
        .route("/api/v0/{story_id}/headshot", post(add_headshot))
        .route("/api/v0/{story_id}/synopsis", post(add_synopsis))
        .route("/api/v0/{story_id}/body", post(add_body))
        .route("/api/v0/{story_id}/paragraph", post(add_paragraph))
        .route("/api/v0/report/{story_id}", post(report_story))
        .with_state(state);

    router
}
