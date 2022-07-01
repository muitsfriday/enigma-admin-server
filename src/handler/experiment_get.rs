use actix_web::{web, web::Json, HttpMessage, HttpRequest};
use serde::{Deserialize, Serialize};

use super::{Claims, CustomAPIError, HandlerError};
use crate::service::experiment as experiment_service;
use crate::Dependency;

#[derive(Deserialize)]
pub struct Params {
    pub id: String,
}

/// Get experimental handler's response payload.
#[derive(Deserialize, Serialize, Debug)]
pub struct ResponsePayload {
    data: experiment_service::Experiment,
}

/// Handle function to handle get experimental request.
pub async fn handle<ER: experiment_service::Store>(
    req: HttpRequest,
    path: web::Path<Params>,
    dep: web::Data<Dependency<ER>>,
) -> Result<Json<ResponsePayload>, CustomAPIError> {
    let experiment_repo = &dep.experiment_repo;
    let params = path.into_inner();

    let ext = req.extensions();
    let channel_id: &str;
    if let Some(ut) = ext.get::<Claims>() {
        channel_id = &ut.channel_id;
    } else {
        return Err(HandlerError::Unauthorize.into());
    }

    let data = experiment_service::get(experiment_repo, &params.id, channel_id).await;

    match data {
        Ok(data) => Ok(Json(ResponsePayload { data })),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::experiment as experiment_service;
    use crate::Dependency;
    use anyhow::Ok;

    use actix_web::{http::header::ContentType, test};

    #[actix_web::test]
    async fn test_handler_ok() {
        let mut mock_store = experiment_service::MockStore::new();
        let mock_get_result = Ok(experiment_service::Experiment::default());
        mock_store
            .expect_get()
            .return_once(move |_, _| mock_get_result);

        let data = web::Data::new(Dependency {
            experiment_repo: mock_store,
        });

        let mock_claims = Claims::default();
        let req = test::TestRequest::default()
            .insert_header(ContentType::json())
            .to_http_request();
        req.extensions_mut().insert(mock_claims);

        let params = web::Path::from(Params {
            id: "aaa".to_owned(),
        });

        let resp = handle(req, params, data).await;
        assert_eq!(resp.is_ok(), true);
    }
}
