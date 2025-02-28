use chrono::{DateTime, Duration , Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug , Clone , Serialize , Deserialize)]
pub enum SendStrategy {
    // Envoi immédiat une seule fois
    Immediate,
    // Envoi différé une seule fois
    Scheduled {
        send_at: DateTime<Utc>,
    },
    // Envoi répété avec intervalle
    Recurring {
        start_at: DateTime<Utc>,
        interval: Duration,
        repeat_count: Option<u32>, // None = infini
        max_attempts: u32,
    },
    // Envoi un lot d'emails avec délai entre chaque lot
    Batch {
        batch_size: usize,
        delay_between_batches: Duration,
        max_batches: Option<u32>,
    }
}