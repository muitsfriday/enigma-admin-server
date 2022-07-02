use std::collections::HashMap;

use actix_web::{web, web::Json, HttpMessage, HttpRequest};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

use super::{Claims, CustomAPIError, HandlerError};
use crate::service::experiment;
use crate::Dependency;

/// Exeriment create handler's request payload struct
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RequestPayload {
    pub name: String,
    pub description: String,
    pub active_interval: Option<Interval>,

    pub variances: Vec<Variance>,
    pub classing: Classing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interval(pub Option<DateTime<Utc>>, pub Option<DateTime<Utc>>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Variance {
    pub group_name: String,
    pub description: String,
    pub indicator: String,
    pub weight: i32,
    pub values: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Classing {
    pub strategy: String,
    pub persistent_mode: String,
}

/// Implement to make the request payload can transform into the service's struct
///

impl From<RequestPayload> for experiment::Experiment {
    fn from(rp: RequestPayload) -> Self {
        Self {
            id: None,
            name: rp.name,
            description: rp.description,
            active_interval: rp.active_interval.map(|v| v.into()),
            variations: rp.variances.into_iter().map(|v| v.into()).collect(),
            classing: rp.classing.into(),
            owner: None,
            channel_id: String::default(),
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}

impl From<Interval> for experiment::Interval {
    fn from(i: Interval) -> Self {
        Self(i.0, i.1)
    }
}

impl From<Classing> for experiment::Classing {
    fn from(c: Classing) -> Self {
        Self {
            strategy: c.strategy,
            persistent_mode: c.persistent_mode,
        }
    }
}

impl From<Variance> for experiment::Variance {
    fn from(v: Variance) -> Self {
        Self {
            group_name: v.group_name,
            description: v.description,
            indicator: v.indicator,
            weight: v.weight,
            values: v.values,
        }
    }
}

/// Create experimental handler's response payload.
#[derive(Deserialize, Serialize, Debug)]
pub struct ResponsePayload {
    data: experiment::Experiment,
}

/// Handle function to handle create experimental request.
pub async fn handle<ER: experiment::Store>(
    req: HttpRequest,
    payload: web::Json<RequestPayload>,
    dep: web::Data<Dependency<ER>>,
) -> Result<Json<ResponsePayload>, CustomAPIError> {
    let mut data: experiment::Experiment = payload.into_inner().into();
    let experiment_repo = &dep.experiment_repo;

    if let Some(ut) = req.extensions().get::<Claims>() {
        let u = serde_json::to_value(ut).unwrap_or_default();
        data.owner = Some(u);
        data.channel_id = ut.channel_id.clone();
    } else {
        return Err(HandlerError::Unauthorize.into());
    }

    let create_result = experiment::create(experiment_repo, data).await;

    match create_result {
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
        let mock_create_result = Ok(String::from("mock"));
        mock_store
            .expect_save()
            .return_once(move |_| mock_create_result);

        let data = web::Data::new(Dependency {
            experiment_repo: mock_store,
        });

        let local_datetime = Utc::now();
        let body = Json(RequestPayload {
            name: "mock-name".to_string(),
            description: "mock-description".to_string(),
            active_interval: Some(Interval(Some(local_datetime), Some(local_datetime))),
            variances: vec![],
            classing: Classing {
                strategy: "mock-value".to_string(),
                persistent_mode: "mock-value".to_string(),
            },
        });

        let mock_claims = Claims::default();

        let req = test::TestRequest::default()
            .insert_header(ContentType::json())
            .to_http_request();
        req.extensions_mut().insert(mock_claims);

        let resp = handle(req, body, data).await;
        assert!(resp.is_ok());
    }
}
