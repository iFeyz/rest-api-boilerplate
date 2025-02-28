use prometheus::{
    Gauge, GaugeVec, IntCounter, IntCounterVec, IntGauge, Registry, 
    opts, register_gauge, register_gauge_vec, register_int_counter, 
    register_int_counter_vec, register_int_gauge
};
use sysinfo::{System, SystemExt, ProcessExt, CpuExt};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

// Structure pour contenir toutes nos métriques
pub struct Metrics {
    pub registry: Registry,
    
    // Métriques système
    pub cpu_usage: Gauge,
    pub memory_usage: Gauge,
    pub memory_total: Gauge,
    pub process_cpu_usage: Gauge,
    pub process_memory_usage: Gauge,
    
    // Métriques d'application
    pub http_requests_total: IntCounterVec,
    pub http_request_duration: GaugeVec,
    pub active_connections: IntGauge,
    pub database_queries_total: IntCounterVec,
    pub database_query_duration: GaugeVec,
    pub email_sent_total: IntCounter,
    pub email_failed_total: IntCounter,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();
        
        // Métriques système
        let cpu_usage = register_gauge!(
            opts!("system_cpu_usage", "Current CPU usage in percent")
        ).unwrap();
        
        let memory_usage = register_gauge!(
            opts!("system_memory_usage_bytes", "Current memory usage in bytes")
        ).unwrap();
        
        let memory_total = register_gauge!(
            opts!("system_memory_total_bytes", "Total system memory in bytes")
        ).unwrap();
        
        let process_cpu_usage = register_gauge!(
            opts!("process_cpu_usage", "Current process CPU usage in percent")
        ).unwrap();
        
        let process_memory_usage = register_gauge!(
            opts!("process_memory_usage_bytes", "Current process memory usage in bytes")
        ).unwrap();
        
        // Métriques d'application
        let http_requests_total = register_int_counter_vec!(
            opts!("http_requests_total", "Total number of HTTP requests"),
            &["method", "path", "status"]
        ).unwrap();
        
        let http_request_duration = register_gauge_vec!(
            opts!("http_request_duration_seconds", "HTTP request duration in seconds"),
            &["method", "path"]
        ).unwrap();
        
        let active_connections = register_int_gauge!(
            opts!("active_connections", "Number of active connections")
        ).unwrap();
        
        let database_queries_total = register_int_counter_vec!(
            opts!("database_queries_total", "Total number of database queries"),
            &["query_type", "table"]
        ).unwrap();
        
        let database_query_duration = register_gauge_vec!(
            opts!("database_query_duration_seconds", "Database query duration in seconds"),
            &["query_type", "table"]
        ).unwrap();
        
        let email_sent_total = register_int_counter!(
            opts!("email_sent_total", "Total number of emails sent")
        ).unwrap();
        
        let email_failed_total = register_int_counter!(
            opts!("email_failed_total", "Total number of failed email sends")
        ).unwrap();
        
        // Enregistrer toutes les métriques dans le registre
        registry.register(Box::new(cpu_usage.clone())).unwrap();
        registry.register(Box::new(memory_usage.clone())).unwrap();
        registry.register(Box::new(memory_total.clone())).unwrap();
        registry.register(Box::new(process_cpu_usage.clone())).unwrap();
        registry.register(Box::new(process_memory_usage.clone())).unwrap();
        registry.register(Box::new(http_requests_total.clone())).unwrap();
        registry.register(Box::new(http_request_duration.clone())).unwrap();
        registry.register(Box::new(active_connections.clone())).unwrap();
        registry.register(Box::new(database_queries_total.clone())).unwrap();
        registry.register(Box::new(database_query_duration.clone())).unwrap();
        registry.register(Box::new(email_sent_total.clone())).unwrap();
        registry.register(Box::new(email_failed_total.clone())).unwrap();
        
        Metrics {
            registry,
            cpu_usage,
            memory_usage,
            memory_total,
            process_cpu_usage,
            process_memory_usage,
            http_requests_total,
            http_request_duration,
            active_connections,
            database_queries_total,
            database_query_duration,
            email_sent_total,
            email_failed_total,
        }
    }
    
    // Démarrer la collecte des métriques système en arrière-plan
    pub async fn start_collector(metrics: Arc<Metrics>) {
        tokio::spawn(async move {
            let mut sys = System::new_all();
            let pid = std::process::id() as usize;
            
            let mut interval = time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                
                // Rafraîchir les informations système
                sys.refresh_all();
                
                // Métriques CPU
                let cpu_usage = sys.global_cpu_info().cpu_usage();
                metrics.cpu_usage.set(cpu_usage as f64);
                
                // Métriques mémoire
                let memory_used = sys.used_memory() as f64;
                let memory_total = sys.total_memory() as f64;
                metrics.memory_usage.set(memory_used);
                metrics.memory_total.set(memory_total);
                
                // Métriques du processus
                if let Some(process) = sys.process(pid.into()) {
                    metrics.process_cpu_usage.set(process.cpu_usage() as f64);
                    metrics.process_memory_usage.set(process.memory() as f64);
                }
            }
        });
    }
}

// Middleware pour mesurer la durée des requêtes HTTP
pub struct RequestMetricsMiddleware {
    metrics: Arc<Metrics>,
}

impl RequestMetricsMiddleware {
    pub fn new(metrics: Arc<Metrics>) -> Self {
        RequestMetricsMiddleware { metrics }
    }
}

impl Clone for RequestMetricsMiddleware {
    fn clone(&self) -> Self {
        RequestMetricsMiddleware {
            metrics: self.metrics.clone(),
        }
    }
}

use actix_web::{
    dev::{Service, Transform, ServiceRequest, ServiceResponse},
    Error,
};
use futures::future::{Ready, ok, LocalBoxFuture};
use std::task::{Context, Poll};
use std::time::Instant;

impl<S, B> Transform<S, ServiceRequest> for RequestMetricsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestMetricsMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestMetricsMiddlewareService {
            service,
            metrics: self.metrics.clone(),
        })
    }
}

pub struct RequestMetricsMiddlewareService<S> {
    service: S,
    metrics: Arc<Metrics>,
}

impl<S, B> Service<ServiceRequest> for RequestMetricsMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Incrémenter le compteur de connexions actives
        self.metrics.active_connections.inc();
        
        let metrics = self.metrics.clone();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let start = Instant::now();
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            // Mesurer la durée de la requête
            let res = fut.await?;
            let duration = start.elapsed().as_secs_f64();
            
            // Enregistrer les métriques
            metrics.http_request_duration
                .with_label_values(&[&method, &path])
                .set(duration);
            
            metrics.http_requests_total
                .with_label_values(&[&method, &path, &res.status().as_u16().to_string()])
                .inc();
            
            // Décrémenter le compteur de connexions actives
            metrics.active_connections.dec();
            
            Ok(res)
        })
    }
}

// Wrapper pour mesurer la durée des requêtes de base de données
pub struct DatabaseMetrics {
    metrics: Arc<Metrics>,
}

impl DatabaseMetrics {
    pub fn new(metrics: Arc<Metrics>) -> Self {
        DatabaseMetrics { metrics }
    }
    
    pub async fn measure_query<F, T>(&self, query_type: &str, table: &str, f: F) -> T
    where
        F: std::future::Future<Output = T>,
    {
        let start = Instant::now();
        
        // Incrémenter le compteur de requêtes
        self.metrics.database_queries_total
            .with_label_values(&[query_type, table])
            .inc();
        
        // Exécuter la requête
        let result = f.await;
        
        // Mesurer la durée
        let duration = start.elapsed().as_secs_f64();
        self.metrics.database_query_duration
            .with_label_values(&[query_type, table])
            .set(duration);
        
        result
    }
}

impl Clone for DatabaseMetrics {
    fn clone(&self) -> Self {
        DatabaseMetrics {
            metrics: self.metrics.clone(),
        }
    }
}

// Wrapper pour les métriques d'email
pub struct EmailMetrics {
    metrics: Arc<Metrics>,
}

impl EmailMetrics {
    pub fn new(metrics: Arc<Metrics>) -> Self {
        EmailMetrics { metrics }
    }
    
    pub fn record_sent_email(&self) {
        self.metrics.email_sent_total.inc();
    }
    
    pub fn record_failed_email(&self) {
        self.metrics.email_failed_total.inc();
    }
}

impl Clone for EmailMetrics {
    fn clone(&self) -> Self {
        EmailMetrics {
            metrics: self.metrics.clone(),
        }
    }
} 