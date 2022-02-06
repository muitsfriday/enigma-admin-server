use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::service::repo::ExperimentRepo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experiment {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub active_interval: Option<Interval>,

    #[serde(with = "ts_milliseconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interval(pub Option<DateTime<Utc>>, pub Option<DateTime<Utc>>);

pub async fn create(
    repo: Box<&dyn ExperimentRepo>,
    data: Experiment,
) -> Result<Experiment, Box<dyn Error>> {
    let mut data = data.clone();
    data.created_at = Some(Utc::now());

    let save_result = repo.save(data.clone()).await?;

    Ok(save_result)
}
