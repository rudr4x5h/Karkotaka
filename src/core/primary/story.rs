use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use super::{
    author::Author, body::Body, domain::Domain, headline::Headline, headshot::Headshot,
    synopsis::Synopsis,
};

#[derive(Serialize, Deserialize)]
pub struct Story {
    id: Ulid,
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
    pub fn new(
        author: Author,
        domain: Domain,
        body: Body,
        headline: Headline,
        synopsis: Synopsis,
        headshot: Headshot,
    ) -> Self {
        Self {
            id: Ulid::new(),
            created_at: Local::now(),
            updated_at: Local::now(),
            author,
            domain,
            body,
            headline,
            synopsis,
            headshot,
        }
    }

    pub fn get_id(&self) -> &Ulid {
        &self.id
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

    pub fn get_headline(&self) -> &Headline {
        &self.headline
    }

    pub fn get_synopsis(&self) -> &Synopsis {
        &self.synopsis
    }

    pub fn get_headshot(&self) -> &Headshot {
        &self.headshot
    }
}

impl Default for Story {
    fn default() -> Self {
        Self::new(
            Author::default(),
            Domain::default(),
            Body::default(),
            Headline::default(),
            Synopsis::default(),
            Headshot::default(),
        )
    }
}
