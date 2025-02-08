use karkotaka::core::{
    primary::story::{Story, StoryWithId},
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
    let story_ids: Vec<String> = story.iter().map(|s| s.get_id().to_string()).collect();
    let story: Vec<Story> = story.iter().map(|s| s.get_story_instance()).collect();

    dbg!(story_ids.clone());

    for (id, s) in story.iter().enumerate() {
        let story_id = story_ids.get(id).unwrap();
        println!("{} - {}", story_id, s.get_headline().get_content())
    }
}
