use anyhow::Error;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use super::{
    primary::{headline::Headline, story::STORY_DB, synopsis::Synopsis},
    utils::{error::AppError, persistence::DB},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Search(String);

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
        let matches = vec![];

        let query = r#"
            SELECT * FROM type::table($table)
            WHERE type::field($field) @@ type::string($terms);
        "#;

        let records = DB
            .query(query)
            .bind(("table", STORY_DB))
            .bind(("field", "headline"))
            .bind(("terms", terms))
            .await
            .expect("Error performing search");

        dbg!(records);

        matches
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FoundStory {
    id: RecordId,
    headline: Headline,
    synopsis: Synopsis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    query: String,
    results: Vec<FoundStory>,
}
