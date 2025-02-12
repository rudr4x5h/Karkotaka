use anyhow::Error;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use super::{
    primary::{
        headline::Headline,
        headshot::Headshot,
        story::{Story, StoryWithId, STORY_DB},
        synopsis::Synopsis,
    },
    utils::{error::AppError, persistence::DB},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Search(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundStory {
    id: RecordId,
    headline: Headline,
    synopsis: Synopsis,
    headshot: Headshot,
}

impl FoundStory {
    pub fn get_id(self) -> RecordId {
        self.id
    }

    pub fn get_headline(self) -> Headline {
        self.headline
    }

    pub fn get_synopsis(self) -> Synopsis {
        self.synopsis
    }

    pub fn get_headshot(self) -> Headshot {
        self.headshot
    }

    pub async fn to_story(self) -> Story {
        let story_id = self.get_id();
        let story: Story = DB.select(story_id).await.unwrap().unwrap();
        story
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    query: String,
    results: Vec<FoundStory>,
}

impl Search {
    pub fn new(query: String) -> Self {
        Self(query)
    }

    fn validate_query(&self) -> Result<(), AppError> {
        if self.0.trim().is_empty() {
            Err(AppError(Error::msg("Empty search terms")))
        } else {
            Ok(())
        }
    }

    pub async fn execute(&self) -> Result<SearchResults, AppError> {
        self.validate_query()?;
        let matches = self.perform_search().await;

        Ok(SearchResults {
            query: self.0.clone(),
            results: matches,
        })
    }

    async fn perform_search(&self) -> Vec<FoundStory> {
        let terms = self.0.clone();
        let mut matches = vec![];

        let query = r#"
            SELECT * FROM type::table($table)
            WHERE headline.content @@ type::string($terms);
        "#;

        let mut records = DB
            .query(query)
            .bind(("table", STORY_DB))
            .bind(("terms", terms))
            .await
            .expect("Error performing search");

        let stories: Vec<StoryWithId> = records.take(0).unwrap();
        let stories_id: Vec<RecordId> = stories.iter().map(|s| s.get_id().to_owned()).collect();
        let stories: Vec<Story> = stories.iter().map(|s| s.get_story_instance()).collect();

        for (idx, story) in stories.iter().enumerate() {
            let id = stories_id.get(idx).unwrap().to_owned();
            let headline = story.get_headline().to_owned();
            let synopsis = story.get_synopsis().to_owned();
            let headshot = story.get_headshot().to_owned();

            let found_story = FoundStory::new(id, headline, synopsis, headshot);
            matches.push(found_story);
        }

        matches
    }
}

impl FoundStory {
    fn new(id: RecordId, headline: Headline, synopsis: Synopsis, headshot: Headshot) -> Self {
        Self {
            id,
            headline,
            synopsis,
            headshot,
        }
    }
}
