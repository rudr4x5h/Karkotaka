use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::RecordId;

use super::image::Image;
use crate::core::search::FoundStory;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenRequest {
    stories: Vec<FoundStory>,
    qty: StoryQuantity,
    img_count: usize,
}

impl GenRequest {
    pub fn get_qty(self) -> StoryQuantity {
        self.qty
    }

    pub fn get_stories(self) -> Vec<FoundStory> {
        self.stories
    }

    pub fn get_image_count(self) -> usize {
        self.img_count
    }
}

impl GenRequest {
    pub fn new(img_count: usize, stories: Vec<FoundStory>) -> Self {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenRequestResponse {
    img_count: usize,
    story_image: HashMap<String, Image>,
}

impl GenRequestResponse {
    pub fn new(img_count: usize, story_image: HashMap<String, Image>) -> Self {
        Self {
            img_count,
            story_image,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StoryQuantity {
    One,
    Many,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
