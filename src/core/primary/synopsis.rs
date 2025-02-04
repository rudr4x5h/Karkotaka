use serde::{Deserialize, Serialize};

use crate::core::secondary::misc::Kind;
use crate::core::secondary::paragraph::Paragraph;

#[derive(Serialize, Deserialize)]
pub struct Synopsis {
    kind: Kind,
    paragraphs: Vec<Paragraph>,
}
