use crate::core::secondary::paragraph::Paragraph;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
pub struct Body {
    num_chars: u8,
    num_words: u8,
    num_para: u8,
    paragraphs: Vec<Paragraph>,
}

impl Body {
    pub fn new() -> Self {
        Self {
            num_chars: 0,
            num_words: 0,
            num_para: 0,
            paragraphs: Vec::new(),
        }
    }

    pub fn from_paras(paragraphs: Vec<Paragraph>) -> Self {
        let mut body = Body::new();
        for para in paragraphs {
            body.add_paragraph(para);
        }
        body
    }

    pub fn get_num_chars(&self) -> u8 {
        self.num_chars
    }

    pub fn get_num_words(&self) -> u8 {
        self.num_words
    }

    pub fn get_num_para(&self) -> u8 {
        self.num_para
    }

    pub fn get_paragraphs(&self) -> &Vec<Paragraph> {
        &self.paragraphs
    }

    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph.clone());
        self.num_chars += paragraph.get_num_chars();
        self.num_words += paragraph.get_num_words();
        self.num_para += 1;
    }

    pub fn remove_paragraph(&mut self, id: &Ulid) {
        let para_idx = self
            .paragraphs
            .iter()
            .position(|p| p.get_id() == id)
            .unwrap();
        let para = self.paragraphs.remove(para_idx);
        self.num_chars -= para.get_num_chars();
        self.num_words -= para.get_num_words();
        self.num_para -= 1;
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}
