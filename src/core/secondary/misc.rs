use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Kind {
    OG,
    AI,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Gender {
    Male,
    Female,
}
