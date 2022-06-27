use std::collections::HashMap;

use actix_web::{web, web::Json, HttpMessage, HttpRequest};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

use super::{CustomAPIError, HandlerError};
use crate::service::experiment;
use crate::Dependency;

/// Exeriment create handler's request payload struct
#[derive(Deserialize, Serialize, Debug)]
pub struct RequestPayload {
    pub name: String,
    pub description: String,
    pub active_interval: Option<Interval>,

    pub variances: Vec<Variance>,
    pub classing: Classing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interval(pub Option<DateTime<Utc>>, pub Option<DateTime<Utc>>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Variance {
    pub group_name: String,
    pub description: String,
    pub indicator: String,
    pub weight: i32,
    pub values: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Classing {
    pub strategy: String,
    pub persistent_mode: String,
}

/// Implement to make the request payload can transform into the service's struct
impl Into<experiment::Experiment> for RequestPayload {
    fn into(self) -> experiment::Experiment {
        experiment::Experiment {
            id: None,
            name: self.name,
            description: self.description,
            active_interval: self.active_interval.map(|i| i.into()),
            variations: self.variances.into_iter().map(|v| v.into()).collect(),
            classing: self.classing.into(),
            owner: None,
            channel_id: String::default(),
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}

impl Into<experiment::Interval> for Interval {
    fn into(self) -> experiment::Interval {
        experiment::Interval(self.0, self.1)
    }
}

impl Into<experiment::Classing> for Classing {
    fn into(self) -> experiment::Classing {
        experiment::Classing {
            strategy: self.strategy,
            persistent_mode: self.persistent_mode,
        }
    }
}

impl Into<experiment::Variance> for Variance {
    fn into(self) -> experiment::Variance {
        experiment::Variance {
            group_name: self.group_name,
            description: self.description,
            indicator: self.indicator,
            weight: self.weight,
            values: self.values,
        }
    }
}

/// Create experimental handler's response payload.
#[derive(Deserialize, Serialize, Debug)]
pub struct ResponsePayload {
    data: experiment::Experiment,
}

/// Handle function to handle create experimental request.
pub async fn handle<ER: experiment::Store>(
    req: HttpRequest,
    payload: web::Json<RequestPayload>,
    dep: web::Data<Dependency<ER>>,
) -> Result<Json<ResponsePayload>, CustomAPIError> {
    let mut data: experiment::Experiment = payload.into_inner().into();
    let experiment_repo = &dep.experiment_repo;

    if let Some(ut) = req.extensions().get::<HashMap<String, serde_json::Value>>() {
        data.owner = ut.get("user").map(|v| v.clone());
    } else {
        return Err(HandlerError::Unauthorize.into());
    }

    let create_result = experiment::create(experiment_repo, data).await;

    match create_result {
        Ok(data) => Ok(Json(ResponsePayload { data })),
        Err(e) => Err(e.into()),
    }
}
