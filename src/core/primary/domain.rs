use serde::{Deserialize, Serialize};

use super::author::Author;

#[derive(Serialize, Deserialize)]
pub struct Domain {
    name: String,
    expert: Author,
}
