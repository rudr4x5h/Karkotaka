use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::core::primary::story::Story;

#[derive(Serialize, Deserialize)]
pub struct Report {
    id: Ulid,
    created_at: DateTime<Local>,
    story: Story,
}

impl Report {
    pub fn new(story: Story) -> Self {
        Self {
            id: Ulid::new(),
            created_at: Local::now(),
            story,
        }
    }

    pub fn get_id(&self) -> &Ulid {
        &self.id
    }

    pub fn get_created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }

    pub fn get_story(&self) -> &Story {
        &self.story
    }
}
