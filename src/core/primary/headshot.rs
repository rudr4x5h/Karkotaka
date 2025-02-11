use serde::{Deserialize, Serialize};

use crate::core::secondary::image::Image;
use crate::core::secondary::misc::Kind;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Headshot {
    kind: Kind,
    images: Vec<Image>,
    image_count: usize,
}

impl Headshot {
    pub fn new(kind: Kind, images: Vec<Image>) -> Self {
        // owning image is necessary, because that image belows to this
        // particular headshot.
        let image_count = images.len();
        Self {
            kind,
            images,
            image_count,
        }
    }

    pub fn get_kind(&self) -> &Kind {
        &self.kind
    }

    pub fn get_image(&self, idx: usize) -> Option<&Image> {
        self.images.get(idx)
    }

    pub fn get_last_image(&self) -> Option<&Image> {
        if self.image_count >= 1 {
            self.images.get(self.image_count - 1)
        } else {
            None
        }
    }

    pub fn add_image(&mut self, image: Image) {
        self.images.push(image);
        self.image_count += 1;
    }

    pub fn pop_image(&mut self, idx: usize) -> Option<Image> {
        if idx >= self.images.len() {
            return None;
        }
        let img = self.images.remove(idx);
        self.image_count -= 1;
        Some(img)
    }

    pub fn set_kind(&mut self, kind: Kind) {
        self.kind = kind;
    }
}

impl Default for Headshot {
    // TODO - Read from config
    fn default() -> Self {
        Self {
            kind: Kind::Placeholder,
            images: vec![Image::new(
                "https://cdn.pixabay.com/photo/2022/08/26/13/15/man-7412527_960_720.png",
            )],
            image_count: 1,
        }
    }
}
