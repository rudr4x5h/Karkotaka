use serde::{Deserialize, Serialize};

use crate::core::secondary::paragraph::Paragraph;

#[derive(Serialize, Deserialize)]
pub struct Body {
    num_chars: u8,
    num_words: u8,
    num_para: u8,
    paragraphs: Vec<Paragraph>,
}
