#![allow(dead_code)]
#![allow(unused_variables)]
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpRequest, HttpResponse, ResponseError,
};
use anyhow::anyhow;
use anyhow::Error as AnyhowError;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::middleware::auth;
use crate::service::experiment;

pub mod experiment_create;
pub mod experiment_get;
pub mod experiment_list;

/// Modify this Claims struct to match up your JWT decoded data.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Claims {
    id: String,
    username: String,
    channel_id: String,
    email: String,
    alias: String,
}

impl auth::ClaimExtractable for Claims {
    fn extract(
        &self,
        v: std::collections::HashMap<String, serde_json::Value>,
    ) -> anyhow::Result<Box<Self>> {
        let seq = v
            .get("user")
            .and_then(|v| v.serialize(serde_json::value::Serializer).ok())
            .ok_or_else(|| anyhow!("error"))?;

        Claims::deserialize(seq)
            .map(Box::new)
            .map_err(|e| anyhow!(e.to_string()))
    }
}

/// Handler error response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    code: String,
    message: String,
}

/// Anyhow wrapper to make anyhow works with actix-web.
#[derive(Display, Debug)]
pub struct CustomAPIError {
    err: AnyhowError,
}

impl ResponseError for CustomAPIError {
    fn status_code(&self) -> StatusCode {
        if let Some(err) = self.err.downcast_ref::<HandlerError>() {
            err.status_code()
        } else if let Some(err) = self.err.downcast_ref::<experiment::UserError>() {
            StatusCode::BAD_REQUEST
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let code = if let Some(err) = self.err.downcast_ref::<HandlerError>() {
            "handle_error".to_string()
        } else if let Some(err) = self.err.downcast_ref::<experiment::UserError>() {
            err.code()
        } else {
            "internal_error".to_string()
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ErrorResponse {
                code,
                message: self.err.to_string(),
            })
    }
}

impl From<AnyhowError> for CustomAPIError {
    fn from(err: anyhow::Error) -> CustomAPIError {
        CustomAPIError { err }
    }
}

impl From<HandlerError> for CustomAPIError {
    fn from(err: HandlerError) -> CustomAPIError {
        CustomAPIError { err: err.into() }
    }
}

impl From<experiment::UserError> for CustomAPIError {
    fn from(err: experiment::UserError) -> CustomAPIError {
        CustomAPIError { err: err.into() }
    }
}

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("unable to access to the resource")]
    Unauthorize,
    #[error("user not authorize to do this action")]
    UnexpectedError,
    #[error("invalid request: `{0}`")]
    BadRequest(String),
    #[error("data not found")]
    NotFound,
}

impl ResponseError for HandlerError {
    fn status_code(&self) -> StatusCode {
        match self {
            HandlerError::Unauthorize => StatusCode::UNAUTHORIZED,
            HandlerError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
            HandlerError::BadRequest(_) => StatusCode::BAD_REQUEST,
            HandlerError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ErrorResponse {
                code: "".to_string(),
                message: self.to_string(),
            })
    }
}

/// Transform json body error into a proper error message.
/// Use for actix_web
pub fn handle_json_error(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let detail = err.to_string();
    let response = match &err {
        error::JsonPayloadError::Deserialize(e) => HttpResponse::BadRequest()
            .content_type("text/plain")
            .json(ErrorResponse {
                code: "invalid_input".to_string(),
                message: e.to_string(),
            }),
        _ => HttpResponse::BadRequest()
            .content_type("text/plain")
            .body(detail),
    };

    error::InternalError::from_response(err, response).into()
}
