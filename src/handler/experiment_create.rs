use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::service::experiment as experiment_serice;
use crate::service::repo;

#[derive(Deserialize)]
pub struct RequestPayload {
    title: String,
    description: String,
}

#[derive(Serialize)]
pub struct ResponsePayload {
    title: String,
    description: String,
}

impl Into<experiment_serice::Experiment> for RequestPayload {
    fn into(self) -> experiment_serice::Experiment {
        experiment_serice::Experiment {
            id: None,
            title: self.title,
            description: self.description,
        }
    }
}

pub async fn handle(
    payload: web::Json<RequestPayload>,
    repo: web::Data<dyn repo::ExperimentRepo>,
) -> impl Responder {
    let data: experiment_serice::Experiment = payload.into_inner().into();
    let r = repo.into_inner().as_ref();
    let result = experiment_serice::create(r, data).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(ResponsePayload {
            title: data.title,
            description: data.description,
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
