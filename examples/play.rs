use karkotaka::core::{
    primary::story::StoryWithId,
    utils::persistence::{init_db_connection, DB},
};

#[tokio::main]
async fn main() {
    init_db_connection().await.expect("Error conecting to DB");

    let terms = "ai";
    let query = r#"
                SELECT * FROM type::table($table)
                WHERE headline.content @@ type::string($terms);
            "#;

    let mut records = DB
        .query(query)
        .bind(("table", "story"))
        .bind(("terms", terms))
        .await
        .unwrap();

    let story: Vec<StoryWithId> = records.take(0).unwrap();

    dbg!(story);
}
