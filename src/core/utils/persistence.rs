use std::sync::LazyLock;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use super::error::AppError;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn init_db_connection() -> Result<(), AppError> {
    DB.connect::<Ws>("localhost:8000").await?;
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("testing").use_db("db_test").await?;
    println!("DB connection established.");
    Ok(())
}
