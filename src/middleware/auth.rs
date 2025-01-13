use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
    body::{BoxBody, EitherBody},
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;

pub struct AuthMiddleware {
    api_key: String,
}

impl AuthMiddleware {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            api_key: self.api_key.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    api_key: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    
        
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        if !req.path().starts_with("/api") {
            return Box::pin(async move {
                let res = svc.call(req).await?;
                Ok(res.map_into_left_body())
            });
        }
        println!("Request: {:?}", req);
        let auth_header = req.headers().get("X-API-Key");
        
        match auth_header {
            Some(auth_value) => {
                if auth_value.as_bytes() == self.api_key.as_bytes() {
                    Box::pin(async move {
                        let res = svc.call(req).await?;
                        Ok(res.map_into_left_body())
                    })
                } else {
                    println!("Invalid API key provided: {:?}", auth_value);
                    let res = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "status": "error",
                            "message": "Invalid API key"
                        }));
                    let (req, _) = req.into_parts();
                    Box::pin(async move { Ok(ServiceResponse::new(req, res).map_into_right_body()) })
                }
            }
            None => {
          
                println!("No API key provided");
                let res = HttpResponse::Unauthorized()
                    .json(serde_json::json!({
                        "status": "error",
                        "message": "API key missing"
                    }));
                let (req, _) = req.into_parts();
                Box::pin(async move { Ok(ServiceResponse::new(req, res).map_into_right_body()) })
            }
        }
    }
}