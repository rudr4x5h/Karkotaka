use karkotaka::core::utils::persistence::{init_db_connection, DB};

#[tokio::main]
async fn main() {
    init_db_connection().await.expect("Error conecting to DB");

    let terms = "aum";
    let query = r#"
                SELECT * FROM story
                WHERE type::field($field) @@ type::string($terms);
            "#;

    let records = DB
        .query(query)
        .bind(("field", "headline"))
        .bind(("terms", terms))
        .await
        .expect("Error performing search");

    dbg!(records);
}
