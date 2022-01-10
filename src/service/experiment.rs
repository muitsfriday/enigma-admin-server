use std::error::Error;

use crate::service::repo::ExperimentRepo;

#[derive(Clone)]
pub struct Experiment {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
}

pub async fn create(
    repo: &impl ExperimentRepo,
    expr: Experiment,
) -> Result<Experiment, Box<dyn Error>> {
    let save_result = repo.save(expr.clone()).await?;

    Ok(save_result)
}
