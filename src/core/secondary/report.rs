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
