use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_http;
use actix_web::{Error, HttpMessage};
use actix_web::body::{MessageBody, ResponseBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::{Future, StreamExt};
use futures::future::{ok, Ready};

// custom request log middleware
pub struct Trace;

impl<S, B> Transform<S> for Trace
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LogMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LogMiddleware {
            service: Rc::new(RefCell::new(service))
        })
    }
}

pub struct LogMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for LogMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let begin = std::time::SystemTime::now();

        let mut svc = self.service.clone();

        Box::pin(async move {
            // request information
            let path = req.path().to_string();
            let method = req.method().as_str().to_string();
            let ip_addr = req.connection_info().remote_addr().unwrap().to_string();
            let queries = req.query_string().to_string();

            // read request body
            let mut body = bytes::BytesMut::new();
            while let Some(chunk) = req.take_payload().next().await {
                body.extend_from_slice(&chunk?);
            }
            let req_body = String::from_utf8(body.clone().to_vec()).unwrap();

            // put bytes back into request body
            let mut payload = actix_http::h1::Payload::empty();
            payload.unread_data(body.freeze());
            req.set_payload(payload.into());

            let mut resp = svc.call(req).await?;

//            log::warn!("{:?}", &resp);

            // read response body
            let mut body = bytes::BytesMut::new();
//            while let Some(chunk) = resp.take_body().next().await {
//                body.extend_from_slice(&chunk?);
//            }
            let resp_body = String::from_utf8(body.clone().to_vec()).unwrap();

            // put bytes back into response body
//            let resp = resp.map_body(move |_, _| {
//                ResponseBody::Body(body.into()).into_body()
//            });

            let duration = begin.elapsed().unwrap().as_millis();

//            log::info!("{} \"{} {}\" ", ip_addr, method, path);
            log::info!("path: {}, method: {}, ip: {}, queries: {}, reqBody: {}, resBody: {}, duration: {}ms",
                    path, method, ip_addr, queries, req_body, resp_body, duration
            );
            Ok(resp)
        })
    }
}
