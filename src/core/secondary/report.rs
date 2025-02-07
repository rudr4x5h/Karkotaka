use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::core::primary::story::Story;

#[derive(Serialize, Deserialize)]
pub struct ReportWithId {
    id: RecordId,
    created_at: DateTime<Local>,
    story: Story,
}

impl ReportWithId {
    pub fn get_id(&self) -> &RecordId {
        &self.id
    }
}

#[derive(Serialize, Deserialize)]
pub struct Report {
    created_at: DateTime<Local>,
    story: Story,
}

impl Report {
    pub fn new(story: Story) -> Self {
        Self {
            created_at: Local::now(),
            story,
        }
    }

    pub fn get_created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }

    pub fn get_story(&self) -> &Story {
        &self.story
    }
}
