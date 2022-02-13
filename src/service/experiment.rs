use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::service::repo::ExperimentRepo;

pub trait Repo {
    fn get(&self, id: &str) -> Result<Experiment, std::io::Error>;
    fn create(&self, data: Experiment) -> Result<Experiment, std::io::Error>;
}

/// Experiment is a struct represents the data model of a single experiment.
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

    pub variations: Vec<Varience>,
    pub group_assign: GroupAssignment,
}

/// Interval is a tuple struct contains information abount datetime range.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interval(pub Option<DateTime<Utc>>, pub Option<DateTime<Utc>>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Varience {
    pub group: String,
    pub description: String,
    pub indicator: String,
    pub weight: i32,
    pub values: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupAssignment {
    pub strategy: String,
    pub persistent: String,
}

/// Get an experiment.
pub async fn get(repo: Box<&dyn ExperimentRepo>, id: &str) -> Result<Experiment, Box<dyn Error>> {
    let experiment = repo.get(id).await?;

    Ok(experiment)
}

/// Create a new experiment.
pub async fn create(
    repo: Box<&dyn ExperimentRepo>,
    data: Experiment,
) -> Result<Experiment, Box<dyn Error>> {
    let mut data = data.clone();
    data.created_at = Some(Utc::now());

    let experiment = repo.save(data.clone()).await?;

    Ok(experiment)
}
