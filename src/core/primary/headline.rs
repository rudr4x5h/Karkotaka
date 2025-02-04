use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::core::secondary::misc::Kind;

#[derive(Serialize, Deserialize)]
pub struct Headline {
    id: Ulid,
    content: String,
    kind: Kind,
}
