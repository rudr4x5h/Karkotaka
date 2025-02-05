use std::collections::HashMap;

use crate::core::primary::story::Story;
use anyhow::Error;
use ulid::Ulid;

pub trait Persist {
    fn save(&mut self, story: Story) -> Result<Ulid, Error>;
    fn load(&mut self, story_id: Ulid) -> Result<&mut Story, Error>;
}

pub struct PersistInMemory {
    db: HashMap<Ulid, Story>,
}

impl PersistInMemory {
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }
}

impl Persist for PersistInMemory {
    fn save(&mut self, story: Story) -> Result<Ulid, Error> {
        let story_id = story.get_id().to_owned();
        self.db.insert(story_id, story);
        Ok(story_id)
    }

    fn load(&mut self, story_id: Ulid) -> Result<&mut Story, Error> {
        if let Some(story) = self.db.get_mut(&story_id) {
            return Ok(story);
        }

        Err(Error::msg("Story not found"))
    }
}
