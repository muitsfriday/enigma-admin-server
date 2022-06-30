use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use derive_more::{Display, Error};
use mockall::automock;
use serde::{Deserialize, Serialize};
use serde_json;
use validator::Validate;

///
/// Defined struct represents the experiment data uses in the service.
///

#[derive(Debug, Default, Validate, Serialize, Deserialize, Clone)]
pub struct Experiment {
    pub id: Option<String>,
    #[validate(length(min = 4, max = 100, message = "must have length between 4 - 100"))]
    pub name: String,
    #[validate(length(max = 500, message = "must have length atmost 500"))]
    pub description: String,
    pub active_interval: Option<Interval>,

    #[validate]
    pub variations: Vec<Variance>,
    pub classing: Classing,

    pub owner: Option<serde_json::Value>,
    pub channel_id: String,

    #[serde(with = "ts_milliseconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interval(pub Option<DateTime<Utc>>, pub Option<DateTime<Utc>>);

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct Variance {
    #[validate(length(min = 1, max = 100, message = "must have length between 1 - 100"))]
    pub group_name: String,
    #[validate(length(max = 500, message = "must have length atmost 500"))]
    pub description: String,
    #[validate(length(min = 1, max = 64, message = "must have length between 1 - 64"))]
    pub indicator: String,
    #[validate(range(min = 0, max = 100, message = "value must bound between 0 - 100"))]
    pub weight: i32,
    pub values: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Classing {
    pub strategy: String,
    pub persistent_mode: String,
}

///
/// Defined error which returns from the service.
///

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "{}", message)]
    ValidationError { message: String },
}

impl UserError {
    pub fn code(&self) -> String {
        match self {
            UserError::ValidationError { message: _ } => "validation_error".to_owned(),
        }
    }
}

#[derive(Debug, Display, Error)]
pub enum StoreError {
    #[display(fmt = "{}", message)]
    InternalError {
        message: String,
    },
    DocumentNotfound,
}

impl StoreError {
    pub fn code(&self) -> String {
        match self {
            StoreError::InternalError { message: _ } => "internal_error".to_owned(),
            StoreError::DocumentNotfound => "document_notfound".to_owned(),
        }
    }
}

///
/// Defined the service contract that abstract away the implementation details.
///
///

#[automock]
#[async_trait]
pub trait Store {
    async fn save(&self, data: &mut Experiment) -> Result<String>;
    async fn list(&self, channel_id: &str) -> Result<Vec<Experiment>>;
}

///
/// Service's interface expose to the other package to use it.
///

pub async fn create(repo: &impl Store, data: Experiment) -> Result<Experiment> {
    let mut data = data.clone();

    if let Err(e) = data.validate() {
        return Err(UserError::ValidationError {
            message: e.to_string(),
        }
        .into());
    }

    match repo.save(&mut data).await {
        Ok(inserted_id) => {
            data.id = Some(inserted_id);

            Ok(data)
        }
        Err(err) => match err.downcast_ref::<StoreError>() {
            Some(_) => todo!(),
            None => todo!(),
        },
    }
}

pub async fn list(repo: &impl Store, channel_id: &str) -> Result<Vec<Experiment>> {
    match repo.list(channel_id).await {
        Ok(experiments) => Ok(experiments),
        Err(err) => Err(err),
    }
}
