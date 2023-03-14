use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::{Error, error, HttpMessage};
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::Future;
use futures::future::{ok, Ready};

use crate::boot::c;
use crate::boot::middleware::jwt::JwtToken;

// custom request auth middleware
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service))
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    //noinspection ALL
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            if match req.headers().get("Authorization")
                .and_then(|h| h.to_str().ok()).and_then(|h| {
                h.split("Bearer").collect::<Vec<&str>>().get(1).map(|w| w.trim())
            }) {
                Some(token) => {
                    match JwtToken::verify(token) {
                        Ok(jwt_token) => {
                            req.extensions_mut().insert(jwt_token).is_none()
                        }
                        Err(_) => false
                    }
                }
                None => false
            } || /* 是否白名单请求 */ c::is_with_list(&req) {
                Ok(svc.call(req).await?)
            } else {
                Err(error::ErrorUnauthorized(serde_json::json!("invalid token")))
            }
        })
    }
}
