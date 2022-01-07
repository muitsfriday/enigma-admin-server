use mongodb::Collection;
use std::error::Error;

use super::ExperimentRepo;

pub struct Data {
    id: Option<String>,
    title: String,
    description: String,
}

pub struct ExperimentMongoRepo<'a> {
    coll: &'a Collection<Data>,
}

impl<'a> ExperimentMongoRepo<'a> {
    pub fn new(coll: &'a Collection<Data>) -> Self {
        ExperimentMongoRepo { coll }
    }
}

impl<'a> ExperimentRepo for ExperimentMongoRepo<'a> {
    fn save(expr: super::Experiment) -> Result<super::Experiment, Box<dyn Error>> {
        Ok(super::Experiment {
            id: None,
            title: "hello".to_owned(),
            description: "world".to_owned(),
        })
    }
}
