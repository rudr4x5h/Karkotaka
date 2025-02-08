use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use super::{
    author::Author, body::Body, domain::Domain, headline::Headline, headshot::Headshot,
    synopsis::Synopsis,
};

pub const STORY_DB: &str = "story";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoryWithId {
    id: RecordId,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    author: Author,
    domain: Domain,
    body: Body,
    headline: Headline,
    synopsis: Synopsis,
    headshot: Headshot,
}

impl StoryWithId {
    pub fn get_id(&self) -> &RecordId {
        &self.id
    }

    pub fn get_body(&self) -> &Body {
        &self.body
    }

    pub fn get_story_instance(&self) -> Story {
        Story {
            created_at: self.created_at,
            updated_at: self.updated_at,
            author: self.author.clone(),
            domain: self.domain.clone(),
            body: self.body.clone(),
            headline: self.headline.clone(),
            synopsis: self.synopsis.clone(),
            headshot: self.headshot.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Story {
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    author: Author,
    domain: Domain,
    body: Body,
    headline: Headline,
    synopsis: Synopsis,
    headshot: Headshot,
}

impl Story {
    pub fn new(headline: Headline) -> Self {
        Self {
            created_at: Local::now(),
            updated_at: Local::now(),
            author: Author::default(),
            domain: Domain::default(),
            body: Body::default(),
            headline,
            synopsis: Synopsis::default(),
            headshot: Headshot::default(),
        }
    }

    pub fn get_author(&self) -> &Author {
        &self.author
    }

    pub fn get_domain(&self) -> &Domain {
        &self.domain
    }

    pub fn get_body(&self) -> &Body {
        &self.body
    }

    pub fn get_body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    pub fn set_body(&mut self, body: Body) {
        self.body = body;
    }

    pub fn merge_body(&mut self, body: Body) {
        let updated_body = self.body.merge(body);
        self.set_body(updated_body);
    }

    pub fn get_headline(&self) -> &Headline {
        &self.headline
    }

    pub fn set_headline(&mut self, headline: Headline) {
        self.headline = headline;
    }

    pub fn get_synopsis(&self) -> &Synopsis {
        &self.synopsis
    }

    pub fn set_synopsis(&mut self, synopsis: Synopsis) {
        self.synopsis = synopsis;
    }

    pub fn get_headshot(&self) -> &Headshot {
        &self.headshot
    }

    pub fn set_headshot(&mut self, headshot: Headshot) {
        self.headshot = headshot;
    }
}

impl Default for Story {
    fn default() -> Self {
        Self::new(Headline::default())
    }
}
