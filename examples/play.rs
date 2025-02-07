use karkotaka::core::primary::story::Story;
use karkotaka::core::utils::persistence::{init_db_connection, DB};

#[tokio::main]
async fn main() {
    init_db_connection().await.expect("Error conecting to DB");

    let terms = "ai";
    let query = r#"
                SELECT * FROM type::table($table)
                WHERE headline.content @@ type::string($terms);
            "#;

    let records = DB
        .query(query)
        .bind(("table", "story"))
        .bind(("terms", terms))
        .await;

    dbg!(records);
}
