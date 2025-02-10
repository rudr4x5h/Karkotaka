use super::persistence::init_db_connection;
use super::{error::AppError, handlers};

use axum::{
    routing::{get, post},
    Router,
};

pub async fn init_server() -> Result<(), AppError> {
    init_db_connection().await?;
    let router_main = init_routes();

    let bind_uri = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(&bind_uri).await.unwrap();
    println!("Listening on: {}", &bind_uri);

    axum::serve(listener, router_main).await.unwrap();
    Ok(())
}

fn init_routes() -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/api/v0/story", post(handlers::create_story))
        .route(
            "/api/v0/story/{story_id}/headshot",
            post(handlers::add_headshot),
        )
        .route(
            "/api/v0/story/{story_id}/synopsis",
            post(handlers::add_synopsis),
        )
        .route("/api/v0/story/{story_id}/body", post(handlers::add_body))
        .route(
            "/api/v0/story/{story_id}/paragraph",
            post(handlers::add_paragraph),
        )
        .route(
            "/api/v0/report/story/{story_id}",
            post(handlers::report_story),
        )
        .route("/api/v0/search", get(handlers::search_stories))
        .route("/api/v0/generate", post(handlers::request_generation))
}
