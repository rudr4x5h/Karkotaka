use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
pub struct Image {
    id: Ulid,
    uri: String,
    caption: String,
    prompt: String,
}
