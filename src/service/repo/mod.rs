use async_trait::async_trait;
use std::error::Error;

pub mod mongo;

pub use mongo::Document as ExperimentMongoDocument;
pub use mongo::ExperimentMongoRepo;

use super::experiment::Experiment;

#[async_trait]
pub trait ExperimentRepo: Sync {
    async fn save(&self, expr: &mut Experiment) -> Result<String, Box<dyn Error>>;

    async fn get(&self, id: &str) -> Result<Experiment, Box<dyn Error>>;

    async fn get_by_group_id(
        &self,
        group_id: &str,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Experiment>, Box<dyn Error>>;
}
