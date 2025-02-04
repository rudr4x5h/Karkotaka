use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use super::misc::Kind;

#[derive(Serialize, Deserialize)]
pub struct Paragraph {
    id: Ulid,
    content: String,
    kind: Kind,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}
