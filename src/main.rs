pub mod core;

use crate::core::utils::endpoints;

#[tokio::main]
async fn main() {
    endpoints::init_server().await.unwrap();
}
