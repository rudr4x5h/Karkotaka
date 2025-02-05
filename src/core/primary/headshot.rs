use serde::{Deserialize, Serialize};

use crate::core::secondary::image::Image;
use crate::core::secondary::misc::Kind;

#[derive(Serialize, Deserialize)]
pub struct Headshot {
    kind: Kind,
    image: Image,
}

impl Headshot {
    pub fn new(kind: Kind, image: Image) -> Self {
        Self { kind, image }
    }

    pub fn get_kind(&self) -> &Kind {
        &self.kind
    }

    pub fn get_image(&self) -> &Image {
        &self.image
    }

    pub fn set_image(&mut self, image: Image) {
        self.image = image;
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
            image: Image::new("file:///home/rudr4x5h/Pictures/Foundry/Karkotaka/logo.png"),
        }
    }
}
