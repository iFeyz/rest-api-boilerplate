use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ready, LocalBoxFuture, Ready};
use std::future::Future;
use std::pin::Pin;

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
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            api_key: self.api_key.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    api_key: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
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
        // Liste des routes qui ne nécessitent pas d'authentification
        let public_paths = vec![
            "/api/email_views/",  // Ajout de la nouvelle route
            "/health",
            "/metrics",
        ];

        // Vérifier si la route actuelle est dans la liste des routes publiques
        let path = req.path();
        if public_paths.iter().any(|public_path| path.starts_with(public_path)) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        // Pour toutes les autres routes, vérifier l'API key
        match req.headers().get("x-api-key") {
            Some(key) if key.to_str().unwrap_or("") == self.api_key => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            _ => {
                Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized("Invalid API key"))
                })
            }
        }
    }
}