use actix_web::{web, web::Json, HttpMessage, HttpRequest};
use serde::{Deserialize, Serialize};

use super::{Claims, CustomAPIError, HandlerError};
use crate::service::experiment;
use crate::Dependency;

/// List user experimental handler's response payload.
#[derive(Deserialize, Serialize, Debug)]
pub struct ResponsePayload {
    data: Vec<experiment::Experiment>,
}

/// Handle function to handle list experimental request.
pub async fn handle<ER: experiment::Store>(
    req: HttpRequest,
    dep: web::Data<Dependency<ER>>,
) -> Result<Json<ResponsePayload>, CustomAPIError> {
    let experiment_repo = &dep.experiment_repo;

    let ext = req.extensions();
    let channel_id: &str;
    if let Some(ut) = ext.get::<Claims>() {
        channel_id = &ut.channel_id;
    } else {
        return Err(HandlerError::Unauthorize.into());
    }

    let data = experiment::list(experiment_repo, channel_id).await;

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
    async fn test_index_ok() {
        let mut mock_store = experiment_service::MockStore::new();
        let mock_list_result = Ok(vec![experiment_service::Experiment::default()]);
        mock_store
            .expect_list()
            .return_once(move |_| mock_list_result);

        let data = web::Data::new(Dependency {
            experiment_repo: mock_store,
        });

        let mock_claims = Claims::default();
        let req = test::TestRequest::default()
            .insert_header(ContentType::json())
            .to_http_request();
        req.extensions_mut().insert(mock_claims);

        let resp = handle(req, data).await;
        assert!(resp.is_ok());
    }
}
