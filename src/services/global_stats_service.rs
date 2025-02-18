use crate::error::ApiError;
use crate::models::global_stats::GlobalStats;
use crate::repositories::global_stats_repository::GlobalStatsRepository;

pub struct GlobalStatsService {
    repository: GlobalStatsRepository,
}

impl GlobalStatsService {
    pub fn new(repository: GlobalStatsRepository) -> Self {
        Self { repository }
    }

    pub async fn get_global_stats(&self) -> Result<GlobalStats, ApiError> {
        self.repository.get_global_stats().await
    }
} 