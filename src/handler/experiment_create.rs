use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

use super::*;
use crate::auth;
use crate::service::experiment;
use crate::service::experiment::Repo as ExperimentRepo;

/// RequestPayload is a struct contains data of request body to the create experiment endpoint.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
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
            owner: None,
            owner_group: String::default(),
        }
    }
}

/// ResponsePayload is a struct that contains the response of the create experiment endpoint.
#[derive(Serialize)]
pub struct ResponsePayload {
    data: Experiment,
}

/// Handle method for create the experiment.
pub async fn handle<T: ExperimentRepo>(
    req: HttpRequest,
    payload: web::Json<RequestPayload>,
    repo: web::Data<T>,
) -> impl Responder {
    info!("Create a new experiment with payload {:?}", payload);

    let mut data: experiment::Experiment = payload.into_inner().into();

    if let Some(ut) = req.extensions().get::<auth::UserToken>() {
        info!("Create a new experiment by {:?}", ut);
        data.owner = Some(ut.user.clone());
        data.owner_group = auth::get_user_group(&ut.user);
    } else {
        warn!("Create experiment without a proper `user` jwt")
    }

    let result = experiment::create(Box::new(repo.into_inner().as_ref()), &mut data).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(ResponsePayload {
            data: Experiment::from(data),
        }),
        Err(err) => {
            error!("Unable to create an experiment");

            return HttpResponse::InternalServerError().body(err.to_string());
        }
    }
}
