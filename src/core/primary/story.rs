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
