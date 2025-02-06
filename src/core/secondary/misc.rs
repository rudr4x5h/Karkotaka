use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Kind {
    OG,
    AI,
    Placeholder,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Gender {
    Male,
    Female,
}
