use crate::core::primary::story::Story;
use anyhow::Error;

pub trait Persist {
    fn save(&self) -> Result<(), Error>;
    fn load(&self) -> Result<(), Error>;
}

pub struct PersistInMemory {
    pub db: Vec<Story>,
}
