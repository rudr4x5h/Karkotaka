use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::core::secondary::misc::Gender;

#[derive(Serialize, Deserialize)]
pub struct Author {
    id: Ulid,
    name: String,
    gender: Gender,
    email: String,
    contact: String,
    profile_img: String,
    profile_uri: String,
}
