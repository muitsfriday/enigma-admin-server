use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::service::{experiment, repo};

#[derive(Deserialize, Debug)]
pub struct RequestPayload {
    title: String,
    description: String,
    active_interval_from: DateTime<Utc>,
    active_interval_to: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct ResponsePayload {
    title: String,
    description: String,
}

impl Into<experiment::Experiment> for RequestPayload {
    fn into(self) -> experiment::Experiment {
        experiment::Experiment {
            id: None,
            title: self.title,
            description: self.description,
            active_interval: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}

pub async fn handle<T: repo::ExperimentRepo>(
    payload: web::Json<RequestPayload>,
    repo: web::Data<T>,
) -> impl Responder {
    let x = payload.into_inner();
    let data: experiment::Experiment = x.into();
    let result = experiment::create(Box::new(repo.into_inner().as_ref()), data).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(ResponsePayload {
            title: data.title,
            description: data.description,
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
