use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Kind {
    OG,
    AI,
}

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize)]
pub enum Quality {
    Optimal,
    Average,
    Deficient,
}
