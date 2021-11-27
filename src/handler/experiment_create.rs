use actix_web::{web, HttpResponse, Responder};

pub struct CreateExperimentRequest {
    title: String,
    description: String,
}

pub async fn handler(
    data: web::Json<CreateExperimentRequest>,
    // experiment_coll: web::Data<Collection<Experiment>>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! {}", data.title))
}
