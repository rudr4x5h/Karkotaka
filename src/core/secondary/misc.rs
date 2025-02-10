use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::core::primary::story::Story;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenRequest {
    stories: Vec<Story>,
    qty: StoryQuantity,
    img_count: usize,
}

impl GenRequest {
    pub fn new(img_count: usize, stories: Vec<Story>) -> Self {
        let qty = if stories.len() > 1 {
            StoryQuantity::One
        } else {
            StoryQuantity::Many
        };

        Self {
            stories,
            qty,
            img_count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StoryQuantity {
    One,
    Many,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Kind {
    OG,
    AI,
    Placeholder,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
    id: RecordId,
}

pub fn get_tuple(record: &RecordId) -> (String, String) {
    let table = record.table().to_owned();
    let key = record.key().to_owned().to_string();
    (table, key)
}

pub fn str_to_recordid(table_key: (String, String)) -> RecordId {
    RecordId::from_table_key(table_key.0, table_key.1)
}

pub fn get_banner() -> String {
    String::from(
        r#"
              __ __           __         __        __
             / //_/___ ______/ /______  / /_____ _/ /______ _
            / ,< / __ `/ ___/ //_/ __ \/ __/ __ `/ //_/ __ `/
           / /| / /_/ / /  / ,< / /_/ / /_/ /_/ / ,< / /_/ /
          /_/ |_\__,_/_/  /_/|_|\____/\__/\__,_/_/|_|\__,_/
    "#,
    )
}
