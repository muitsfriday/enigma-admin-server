use async_trait::async_trait;
use std::error::Error;

mod mongo;

pub use mongo::Document as ExperimentMongoDocument;
pub use mongo::ExperimentMongoRepo;

use super::experiment::Experiment;

#[async_trait]
pub trait ExperimentRepo {
    async fn save(&self, expr: Experiment) -> Result<Experiment, Box<dyn Error>>;
}
