use std::{
    collections::HashMap,
    future::{ready, Ready},
};

use actix_web::error::ErrorUnauthorized;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JwtExtractor {
    jwt_secret: String,
}

impl JwtExtractor {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for JwtExtractor
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JWTExtractorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTExtractorMiddleware {
            service,
            jwt_secret: self.jwt_secret.clone(),
        }))
    }
}

pub struct JWTExtractorMiddleware<S> {
    jwt_secret: String,
    service: S,
}

impl<S, B> Service<ServiceRequest> for JWTExtractorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
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
                        req.extensions_mut().insert(token_data.claims);
                        is_auth_pass = true;
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
