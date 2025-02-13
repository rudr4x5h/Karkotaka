use anyhow::anyhow;
use karkotaka::core::{
    plug::llm::{clean_json, GeneratedSynopsis},
    primary::story::Story,
    secondary::misc::str_to_recordid,
    utils::{
        error::AppError,
        persistence::{init_db_connection, DB},
    },
};

#[tokio::main]
async fn main() {
    // init_db_connection().await.expect("Error conecting to DB");

    // let terms = "ai";
    // let query = r#"
    //             SELECT * FROM type::table($table)
    //             WHERE headline.content @@ type::string($terms);
    //         "#;

    // let mut records = DB
    //     .query(query)
    //     .bind(("table", "story"))
    //     .bind(("terms", terms))
    //     .await
    //     .unwrap();

    // let story: Vec<StoryWithId> = records.take(0).unwrap();
    // let story_ids: Vec<String> = story.iter().map(|s| s.get_id().to_string()).collect();
    // let story: Vec<Story> = story.iter().map(|s| s.get_story_instance()).collect();

    // dbg!(story_ids.clone());

    // for (id, s) in story.iter().enumerate() {
    //     let story_id = story_ids.get(id).unwrap();
    //     println!("{} - {}", story_id, s.get_headline().get_content())
    // }
    //

    init_db_connection().await.expect("Error connecting to db");
    let story_id = str_to_recordid(("story".to_string(), "qcvhj1a7wahhryzst22p".to_string()));

    let story: Story = DB.select(story_id).await.unwrap().unwrap();

    dbg!(story.get_synopsis().exists_generated_para());
}
