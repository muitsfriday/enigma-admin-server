use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateExperimentRequest {
    title: String,
    description: String,
}

pub async fn handle(data: web::Json<CreateExperimentRequest>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! {} {}", data.title, data.description))
}
