pub mod core;

use karkotaka::core::secondary::misc::get_banner;

use crate::core::utils::endpoints;

#[tokio::main]
async fn main() {
    println!("{}", get_banner());
    endpoints::init_server().await.unwrap();
}
