use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

use crate::service::{experiment, repo};

/// RequestPayload is a struct contains data of request body to the create experiment endpoint.
///
#[derive(Deserialize, Debug)]
pub struct RequestPayload {
    title: String,
    description: String,

    active_interval_from: Option<DateTime<Utc>>,
    active_interval_to: Option<DateTime<Utc>>,

    variations: Vec<Varience>,
    group_assign: GroupAssignment,
}

/// Varience is a struct contains data of each variance in experiment payload.
#[derive(Deserialize, Debug)]
pub struct Varience {
    pub group: String,
    pub description: String,
    pub indicator: String,
    pub weight: i32,
    pub values: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct GroupAssignment {
    pub strategy: String,
    pub persistent: String,
}

/// ResponsePayload is a struct that contains the response of the create experiment endpoint.
#[derive(Serialize)]
pub struct ResponsePayload {
    id: Option<String>,
    title: String,
    description: String,
    active_interval: Option<experiment::Interval>,
    variations: Vec<experiment::Varience>,
    group_assign: experiment::GroupAssignment,
}

/// Implementation for transforms request body to service's data model.
impl From<RequestPayload> for experiment::Experiment {
    fn from(val: RequestPayload) -> Self {
        let active_interval = Some(experiment::Interval(
            val.active_interval_from,
            val.active_interval_to,
        ));

        let vs = val
            .variations
            .iter()
            .map(|v| experiment::Varience {
                group: v.group.to_owned(),
                description: v.description.to_owned(),
                indicator: v.indicator.to_owned(),
                weight: v.weight.to_owned(),
                values: v.values.to_owned(),
            })
            .collect();

        experiment::Experiment {
            id: None,
            title: val.title,
            description: val.description,
            active_interval,
            variations: vs,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            group_assign: experiment::GroupAssignment {
                strategy: val.group_assign.strategy,
                persistent: val.group_assign.persistent,
            },
        }
    }
}

/// Handle method for create the experiment.
pub async fn handle<T: repo::ExperimentRepo>(
    payload: web::Json<RequestPayload>,
    repo: web::Data<T>,
) -> impl Responder {
    let x = payload.into_inner();
    let data: experiment::Experiment = x.into();
    let result = experiment::create(Box::new(repo.into_inner().as_ref()), data).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(ResponsePayload {
            id: data.id,
            title: data.title,
            description: data.description,
            active_interval: data.active_interval,
            variations: data.variations,
            group_assign: data.group_assign,
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
