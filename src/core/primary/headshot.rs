use serde::{Deserialize, Serialize};

use crate::core::secondary::image::Image;
use crate::core::secondary::misc::Kind;

#[derive(Serialize, Deserialize)]
pub struct Headshot {
    kind: Kind,
    image: Image,
}
