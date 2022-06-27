use std::collections::HashMap;

use actix_web::{web, web::Json, HttpMessage, HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json;

use super::{CustomAPIError, HandlerError};
use crate::service::experiment;
use crate::Dependency;

/// Create experimental handler's response payload.
#[derive(Deserialize, Serialize, Debug)]
pub struct ResponsePayload {
    data: Vec<experiment::Experiment>,
}

/// Handle function to handle create experimental request.
pub async fn handle<ER: experiment::Store>(
    req: HttpRequest,
    dep: web::Data<Dependency<ER>>,
) -> Result<Json<ResponsePayload>, CustomAPIError> {
    let experiment_repo = &dep.experiment_repo;

    let ext = req.extensions();
    let channel_id;
    if let Some(ut) = ext.get::<HashMap<String, serde_json::Value>>() {
        channel_id = ut
            .get("user")
            .and_then(|v| v.get("channel_id"))
            .and_then(|v| v.as_str())
            .ok_or::<CustomAPIError>(HandlerError::Unauthorize.into())?
    } else {
        return Err(HandlerError::Unauthorize.into());
    }

    let data = experiment::list(experiment_repo, channel_id).await;

    match data {
        Ok(data) => Ok(Json(ResponsePayload { data })),
        Err(e) => Err(e.into()),
    }
}
