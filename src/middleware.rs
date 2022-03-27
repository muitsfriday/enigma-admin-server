use log::error;
use std::pin::Pin;
use std::result::Result;

use actix_service::{Service, Transform};
use actix_web::HttpMessage;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::auth;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Jwt {
    jwt_secret: String,
}

impl Jwt {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Jwt
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddleware {
            service,
            jwt_secret: self.jwt_secret.clone(),
        })
    }
}

pub struct JwtMiddleware<S> {
    jwt_secret: String,
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_service::forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(authen_header) = req.headers().get("Authorization") {
            if let Ok(authen_str) = authen_header.to_str() {
                if authen_str.to_lowercase().starts_with("bearer") {
                    let token = authen_str[6..authen_str.len()].trim();

                    let token =
                        decode_token(token, self.jwt_secret.as_ref()).unwrap_or_else(|err| {
                            error!("Unable to unwrap jwt uyser token {}", err);

                            auth::UserToken::default()
                        });

                    req.extensions_mut().insert(token);
                }
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

fn decode_token(token: &str, secret: &str) -> Result<auth::UserToken, jsonwebtoken::errors::Error> {
    let td = decode::<auth::UserToken>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(td.claims)
}
