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
pub struct Jwt<'a> {
    jwt_secret: &'a str,
}

impl<'a> Jwt<'a> {
    pub fn new(jwt_secret: &'a str) -> Self {
        Self { jwt_secret }
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<'a, S, B> Transform<S, ServiceRequest> for Jwt<'static>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddleware<'static, S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddleware {
            service,
            jwt_secret: self.jwt_secret,
        })
    }
}

pub struct JwtMiddleware<'a, S> {
    jwt_secret: &'a str,
    service: S,
}

impl<'a, S, B> Service<ServiceRequest> for JwtMiddleware<'a, S>
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
        println!("--------------------- Request Accepted ----------------------");
        println!("--- Hi from start. You requested: {}", req.path());

        if let Some(authen_header) = req.headers().get("Authorization") {
            if let Ok(authen_str) = authen_header.to_str() {
                if authen_str.to_lowercase().starts_with("bearer") {
                    let token = authen_str[6..authen_str.len()].trim();
                    println!("--- Request Token: {}", token);

                    let token = decode_token(token, self.jwt_secret).unwrap_or_else(|err| {
                        println!("Token Error: {}", err);

                        auth::UserToken::default()
                    });

                    println!("Access as User: {:?}", token.user);
                    req.extensions_mut().insert(token.user);
                }
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
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
