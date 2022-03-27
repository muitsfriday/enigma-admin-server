use async_trait::async_trait;
use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use super::Context;
use crate::service::repo::ExperimentRepo;

/// Trait `Repo` represent an ablity which reflect the experiment repository.
/// The following method is to be implement for do the CRUD job to the persistent database.
#[async_trait]
pub trait Repo: Sync {
    async fn save(&self, expr: &mut Experiment) -> Result<String, Box<dyn Error>>;

    async fn get(&self, id: &str) -> Result<Experiment, Box<dyn Error>>;

    async fn get_by_group_id(
        &self,
        group_id: &str,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Experiment>, Box<dyn Error>>;
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

    pub owner: Option<HashMap<String, serde_json::Value>>,
    pub owner_group: String,
}

/// Interval is a tuple struct contains information abount datetime range.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interval(pub Option<DateTime<Utc>>, pub Option<DateTime<Utc>>);

///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Varience {
    pub group: String,
    pub description: String,
    pub indicator: String,
    pub weight: i32,
    pub values: HashMap<String, serde_json::Value>,
}

///
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

///
///
///
pub async fn list<'a>(
    context: Context<'a>,
    repo: Box<&dyn Repo>,
    page: usize,
    limit: usize,
) -> Result<Vec<Experiment>, Box<dyn Error>> {
    let group_id = context.user_group_id;

    let experiments = repo
        .get_by_group_id(group_id, (page - 1) * limit, limit)
        .await?;

    Ok(experiments)
}

/// Create an new experiment.
pub async fn create(repo: Box<&dyn Repo>, data: &mut Experiment) -> Result<String, Box<dyn Error>> {
    data.created_at = Some(Utc::now());

    let id = repo.save(data).await?;

    Ok(id)
}
