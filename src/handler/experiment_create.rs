use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::*;
use crate::service::{experiment, repo};

/// RequestPayload is a struct contains data of request body to the create experiment endpoint.
///
#[derive(Deserialize, Debug)]
pub struct RequestPayload {
    title: String,
    description: String,
    active_interval: Option<Interval>,
    variations: Vec<Varience>,
    group_assign: GroupAssignment,
}

impl Into<experiment::Experiment> for RequestPayload {
    fn into(self) -> experiment::Experiment {
        let active_interval = match self.active_interval {
            Some(act) => Some(experiment::Interval(act.0, act.1)),
            None => None,
        };
        let vs: Vec<experiment::Varience> = self.variations.into_iter().map(|v| v.into()).collect();

        experiment::Experiment {
            id: None,
            title: self.title.clone(),
            description: self.description.clone(),
            active_interval: active_interval,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            variations: vs,
            group_assign: self.group_assign.into(),
        }
    }
}

/// ResponsePayload is a struct that contains the response of the create experiment endpoint.
#[derive(Serialize)]
pub struct ResponsePayload {
    data: Experiment,
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
            data: Experiment::from(data),
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
