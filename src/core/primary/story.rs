use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use super::{
    author::Author, body::Body, domain::Domain, headline::Headline, headshot::Headshot,
    synopsis::Synopsis,
};

#[derive(Serialize, Deserialize, Clone)]
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
    pub fn new(headline: Headline) -> Self {
        Self {
            id: Ulid::new(),
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
