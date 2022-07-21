use actix_web::{web, web::Json, HttpMessage, HttpRequest};
use serde::{Deserialize, Serialize};

use super::{Claims, CustomAPIError, HandlerError};
use crate::service::experiment as experiment_service;
use crate::Dependency;

#[derive(Deserialize)]
pub struct Params {
    pub id: String,
}

#[derive(Serialize)]
pub struct ResponsePayload {
    acknowledge: bool,
}

/// Handle function to handle delete experimental request.
pub async fn handle<ER: experiment_service::Store>(
    req: HttpRequest,
    path: web::Path<Params>,
    dep: web::Data<Dependency<ER>>,
) -> Result<Json<ResponsePayload>, CustomAPIError> {
    let experiment_repo = &dep.experiment_repo;
    let params = path.into_inner();

    let channel_id: String;
    if let Some(ut) = req.extensions().get::<Claims>() {
        channel_id = ut.channel_id.clone();
    } else {
        return Err(HandlerError::Unauthorize.into());
    }

    let data = experiment_service::delete(experiment_repo, &params.id, &channel_id).await;

    match data {
        Ok(data) => Ok(Json(ResponsePayload { acknowledge: true })),
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
        let mock_delete_result = Ok(());
        mock_store
            .expect_delete()
            .return_once(move |_, _| mock_delete_result);

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
        assert!(resp.is_ok());
    }
}
