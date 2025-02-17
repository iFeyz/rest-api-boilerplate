use sqlx::PgPool;
use crate::error::ApiError;
use crate::models::campaign_stats::*;

#[derive(Clone)]
pub struct CampaignStatsRepository {
    pool: PgPool,
}

impl CampaignStatsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_campaign_stats(&self, campaign_id: i32) -> Result<CampaignStats, ApiError> {
        // Get campaign info
        let campaign = sqlx::query!(
            r#"
            SELECT 
                name, 
                status as "status!: String",
                started_at
            FROM campaigns
            WHERE id = $1
            "#,
            campaign_id
        )
        .fetch_one(&self.pool)
        .await?;

        // Get overall stats with correct subscriber and sent counts
        let overall_stats = sqlx::query!(
            r#"
            WITH subscriber_counts AS (
                SELECT COUNT(DISTINCT sl.subscriber_id) as total_subscribers
                FROM campaign_lists cl
                JOIN subscriber_lists sl ON cl.list_id = sl.list_id
                WHERE cl.campaign_id = $1
            ),
            email_stats AS (
                SELECT 
                    COUNT(DISTINCT ev.id) as total_opens,
                    COUNT(DISTINCT ev.subscriber_id) as unique_opens,
                    CAST((SELECT sent FROM campaigns WHERE id = $1) AS bigint) as total_sent
                FROM sequence_emails se
                LEFT JOIN email_views ev ON se.id = ev.sequence_email_id
                WHERE se.campaign_id = $1
            )
            SELECT 
                sc.total_subscribers,
                es.total_sent,
                es.total_opens,
                es.unique_opens,
                CASE 
                    WHEN es.total_sent > 0 
                    THEN (CAST(es.unique_opens AS float8) / CAST(es.total_sent AS float8) * 100.0)
                    ELSE 0.0 
                END as "open_rate!: f64"
            FROM subscriber_counts sc
            CROSS JOIN email_stats es
            "#,
            campaign_id
        )
        .fetch_one(&self.pool)
        .await?;

        // Get sequence email stats with per-sequence open rates
        let sequence_stats = sqlx::query_as!(
            SequenceEmailStats,
            r#"
            WITH subscriber_count AS (
                SELECT COUNT(DISTINCT sl.subscriber_id) as total_subscribers
                FROM campaign_lists cl
                JOIN subscriber_lists sl ON cl.list_id = sl.list_id
                WHERE cl.campaign_id = $1
            ),
            stats AS (
                SELECT 
                    se.id as sequence_email_id,
                    se.position,
                    se.subject,
                    se.status as "status!: String",
                    se.send_at as sent_at,
                    (SELECT total_subscribers FROM subscriber_count) as "total_sent!",
                    COUNT(ev.id) as "total_opens!",
                    COUNT(DISTINCT ev.subscriber_id) as "unique_opens!",
                    CASE 
                        WHEN (SELECT total_subscribers FROM subscriber_count) > 0 
                        THEN (CAST(COUNT(DISTINCT ev.subscriber_id) AS float8) / 
                              CAST((SELECT total_subscribers FROM subscriber_count) AS float8) * 100.0)
                        ELSE 0.0 
                    END as "open_rate!: f64"
                FROM sequence_emails se
                LEFT JOIN email_views ev ON se.id = ev.sequence_email_id
                WHERE se.campaign_id = $1
                GROUP BY se.id, se.position, se.subject, se.status, se.send_at
                ORDER BY se.position
            )
            SELECT * FROM stats
            "#,
            campaign_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(CampaignStats {
            campaign_id,
            campaign_name: campaign.name,
            status: campaign.status,
            start_date: campaign.started_at,
            total_subscribers: overall_stats.total_subscribers.unwrap_or(0),
            total_sent: overall_stats.total_sent.unwrap_or(0),
            total_opens: overall_stats.total_opens.unwrap_or(0),
            unique_opens: overall_stats.unique_opens.unwrap_or(0),
            open_rate: overall_stats.open_rate,
            total_sequence_emails: sequence_stats.len() as i64,
            sequence_stats,
            country_stats: vec![], // Implement if needed
            city_stats: vec![], // Implement if needed
            opens_by_hour: vec![], // Implement if needed
            opens_by_day: vec![], // Implement if needed
        })
    }
}

// ... rest of the implementation ... 