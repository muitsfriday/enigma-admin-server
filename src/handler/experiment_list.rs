use actix_web::error::{Error, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use log::info;
use serde::{Deserialize, Serialize};

use crate::auth;
use crate::service::experiment;
use crate::service::experiment::Repo as ExperimentRepo;
use crate::service::Context as ServiceContext;
use crate::AppData;

use super::*;

#[derive(Deserialize, Debug)]
pub struct ListingArgs {
    page: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponsePayload {
    data: Vec<Experiment>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorPayload<'a> {
    error: &'a str,
}

/// Handle method for create the experiment.
pub async fn handle<T: ExperimentRepo>(
    req: HttpRequest,
    repo: web::Data<T>,
    app_data: web::Data<AppData>,
    qs: web::Query<ListingArgs>,
) -> Result<HttpResponse, Error> {
    let page = qs.into_inner().page.unwrap_or(1);
    let ext = req.extensions();
    let user_token = ext
        .get::<auth::UserToken>()
        .ok_or_else(|| ErrorUnauthorized("Cannot determine group_id from a given JWT."))?;

    let req_context = ServiceContext {
        user_group_id: user_token.get_user_group().unwrap_or_default(),
        user_id: user_token.get_user_id().unwrap_or_default(),
    };
    let limit = app_data.experiment_item_per_page;

    info!(
        "List experiment of user group id = {:?}, page = {}, limit = {}",
        req_context, page, limit
    );

    let result = experiment::list(
        req_context,
        Box::new(repo.into_inner().as_ref()),
        page,
        limit,
    )
    .await;

    result
        .map(|es| {
            HttpResponse::Ok().json(ResponsePayload {
                data: es.into_iter().map(|e| Experiment::from(e)).collect(),
            })
        })
        .map_err(|err| ErrorInternalServerError(err.to_string()))
}
