use anyhow::Error;
use axum::{routing::get, Router};

pub async fn init_server() -> Result<(), Error> {
    let router_main = Router::new().route("/", get(|| async { "Namaskaram" }));

    let bind_uri = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(&bind_uri).await.unwrap();
    println!("Listening on: {}", &bind_uri);
    axum::serve(listener, router_main).await.unwrap();

    Ok(())
}
