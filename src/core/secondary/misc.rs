use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Kind {
    OG,
    AI,
    Placeholder,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
    id: RecordId,
}

pub fn get_tuple(record: &RecordId) -> (String, String) {
    let table = record.table().to_owned();
    let key = record.key().to_owned().to_string();
    (table, key)
}

pub fn str_to_recordid(table_key: (String, String)) -> RecordId {
    let record = RecordId::from_table_key(table_key.0, table_key.1);
    record
}
