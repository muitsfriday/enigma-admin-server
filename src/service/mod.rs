use serde::{Deserialize, Serialize};

pub mod experiment;
pub mod repo;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub alias: String,
}
