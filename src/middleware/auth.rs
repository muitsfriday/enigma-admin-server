use std::{
    collections::HashMap,
    future::{ready, Ready},
};

use actix_web::error::ErrorUnauthorized;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use anyhow::Result;
use dyn_clone::DynClone;
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub trait ClaimExtractable: DynClone {
    fn extract(&self, v: HashMap<String, serde_json::Value>) -> Result<Box<Self>>;
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JwtExtractor<T: ClaimExtractable + Clone> {
    jwt_secret: String,
    claims: T,
}

impl<T> JwtExtractor<T>
where
    T: ClaimExtractable + Clone,
{
    pub fn new(jwt_secret: String, c: T) -> Self {
        Self {
            jwt_secret,
            claims: c,
        }
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B, T> Transform<S, ServiceRequest> for JwtExtractor<T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    T: ClaimExtractable + Clone + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JWTExtractorMiddleware<S, T>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTExtractorMiddleware {
            service,
            jwt_secret: self.jwt_secret.clone(),
            claims: self.claims.clone(),
        }))
    }
}

pub struct JWTExtractorMiddleware<S, T: ClaimExtractable> {
    jwt_secret: String,
    service: S,
    claims: T,
}

impl<S, B, T> Service<ServiceRequest> for JWTExtractorMiddleware<S, T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    T: ClaimExtractable + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut is_auth_pass = false;

        if let Some(authen_header) = req.headers().get("Authorization") {
            if let Ok(authen_str) = authen_header.to_str() {
                if authen_str.to_lowercase().starts_with("bearer") {
                    let token = authen_str[6..authen_str.len()].trim();
                    let header_data = decode::<HashMap<String, serde_json::Value>>(
                        token,
                        &DecodingKey::from_secret(self.jwt_secret.as_ref()),
                        &Validation::new(Algorithm::HS256),
                    );

                    if let Ok(token_data) = header_data {
                        let claims = self.claims.extract(token_data.claims);
                        if let Ok(result) = claims {
                            req.extensions_mut().insert(*result);
                            is_auth_pass = true;
                        }
                    }
                }
            }
        }

        if !is_auth_pass {
            return Box::pin(async move { Err(ErrorUnauthorized("not authorized"))? });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}
