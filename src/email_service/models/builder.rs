use crate::email_service::{
    models::{EmailRequest, SendStrategy},
    error::EmailError,
    provider::EmailProvider,
};

use chrono::{DateTime, Duration , Utc};

pub struct EmailBuilder {
    request: EmailRequest,
    strategy: SendStrategy,
    provider_name: Option<String>,
}

impl EmailBuilder {
    pub fn new(request: EmailRequest) -> Self {
        Self {
            request,
            strategy: SendStrategy::Immediate,
            provider_name: None,
        }
    }

    // Configuration du provider
    pub fn with_provider(mut self, provider_name: String) -> Self {
        self.provider_name = Some(provider_name);
        self
    }

    // Envoi programmé
    pub fn schedule_at(mut self, send_at: DateTime<Utc>) -> Self {
        self.strategy = SendStrategy::Scheduled { send_at };
    }

    // Envoi récurrent
    pub fn recurring(
        mut self,
        start_at: DateTime<Utc>,
        interval: Duration,
        repeat_count: Option<u32>,
        max_attempts: u32,
    ) -> Self {
        self.strategy = SendStrategy::Recurring {
            start_at,
            interval,
            repeat_count,
            max_attempts,
        };
        self
    }

    //Envoi par lots
    pub fn batch(
        mut self,
        batch_size: usize,
        delay_between_batches: Duration,
        max_batches: Option<u32>,
    ) -> Self {
        self.strategy = SendStrategy::Batch {
            batch_size,
            delay_between_batches,
            max_batches,
        };
        self
    }

}