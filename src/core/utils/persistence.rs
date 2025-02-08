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

    create_search_analyzers().await?;
    create_search_indices().await?;
    Ok(())
}

pub async fn create_search_analyzers() -> Result<(), AppError> {
    let query = r#"
        DEFINE ANALYZER IF NOT EXISTS primary_search_analyzer
        TOKENIZERS blank, punct
        FILTERS ascii, lowercase, snowball(english);
    "#;

    DB.query(query).await?.check()?;
    Ok(())
}

pub async fn create_search_indices() -> Result<(), AppError> {
    let sql = r#"
            DEFINE INDEX IF NOT EXISTS headline
            ON TABLE story
            FIELDS headline.content SEARCH
            ANALYZER primary_search_analyzer BM25;
            "#;

    DB.query(sql).await?.check()?;

    Ok(())
}
