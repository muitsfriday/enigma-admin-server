use std::io::Error as ioError;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::*;
use crate::service::experiment::Repo as ExperimentRepo;

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponsePayload {
    data: Experiment,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorPayload<'a> {
    error: &'a str,
}

/// Handle method for create the experiment.
pub async fn handle<T: ExperimentRepo>(
    repo: web::Data<T>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let result = repo.get(&id).await;
    let unhandle_error = HttpResponse::InternalServerError().json(ErrorPayload {
        error: "unexpected error",
    });

    match result {
        Ok(data) => HttpResponse::Ok().json(ResponsePayload {
            data: Experiment::from(data),
        }),
        Err(err) => match err.downcast::<ioError>() {
            Ok(err) => match err.kind() {
                std::io::ErrorKind::NotFound => HttpResponse::NotFound().json(ErrorPayload {
                    error: &err.to_string(),
                }),
                _ => unhandle_error,
            },
            Err(_) => unhandle_error,
        },
    }
}
