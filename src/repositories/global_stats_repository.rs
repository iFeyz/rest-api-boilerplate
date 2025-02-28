use sqlx::PgPool;
use crate::error::ApiError;
use crate::models::global_stats::*;
use sqlx::types::BigDecimal;
use num_traits::ToPrimitive;

#[derive(Clone)]
pub struct GlobalStatsRepository {
    pool: PgPool,
}

impl GlobalStatsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_global_stats(&self) -> Result<GlobalStats, ApiError> {
        // Get overall numbers
        let overall = sqlx::query!(
            r#"
            WITH subscriber_stats AS (
                SELECT 
                    COUNT(*) as total_subscribers,
                    COUNT(*) FILTER (WHERE status = 'enabled') as active_subscribers,
                    COUNT(*) FILTER (WHERE DATE(created_at) = CURRENT_DATE) as new_today,
                    COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE - INTERVAL '7 days') as new_this_week,
                    COUNT(*) FILTER (WHERE created_at >= DATE_TRUNC('month', CURRENT_DATE)) as new_this_month
                FROM subscribers
            ),
            campaign_stats AS (
                SELECT 
                    COUNT(*) as total_campaigns,
                    COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE - INTERVAL '30 days') as last_30_days,
                    COUNT(*) FILTER (WHERE status = 'draft') as active_campaigns,
                    COUNT(*) FILTER (WHERE status = 'finished') as completed_campaigns,
                    COUNT(*) FILTER (WHERE status = 'cancelled') as failed_campaigns,
                    SUM(sent) as total_emails_sent_unique,
                    (
                        SELECT COALESCE(SUM(c.sent * list_recipients.recipient_count), 0)
                        FROM campaigns c
                        LEFT JOIN (
                            SELECT cl.campaign_id, COUNT(sl.subscriber_id) as recipient_count
                            FROM campaign_lists cl
                            JOIN subscriber_lists sl ON cl.list_id = sl.list_id
                            WHERE sl.status = 'confirmed'
                            GROUP BY cl.campaign_id
                        ) list_recipients ON c.id = list_recipients.campaign_id
                    ) as total_emails_sent
                FROM campaigns
            ),
            list_stats AS (
                SELECT 
                    COUNT(*) as total_lists,
                    COUNT(*) FILTER (WHERE type = 'public') as active_lists,
                    AVG(subscriber_count) as avg_list_size,
                    MAX(subscriber_count) as max_list_size
                FROM (
                    SELECT l.id, l.type, COUNT(sl.subscriber_id) as subscriber_count
                    FROM lists l
                    LEFT JOIN subscriber_lists sl ON l.id = sl.list_id
                    GROUP BY l.id, l.type
                ) list_counts
            ),
            engagement_stats AS (
                SELECT 
                    COUNT(*) as total_opens,
                    COUNT(*) FILTER (WHERE DATE(opened_at) = CURRENT_DATE) as opens_today,
                    COUNT(*) FILTER (WHERE opened_at >= CURRENT_DATE - INTERVAL '7 days') as opens_this_week,
                    COUNT(*) FILTER (WHERE opened_at >= DATE_TRUNC('month', CURRENT_DATE)) as opens_this_month,
                    AVG(campaign_opens) as avg_opens_per_campaign
                FROM email_views ev
                CROSS JOIN (
                    SELECT campaign_id, COUNT(*) as campaign_opens
                    FROM email_views
                    GROUP BY campaign_id
                ) campaign_opens
            ),
            unsubscribe_stats AS (
                SELECT COUNT(*) as unsubscribes_this_month
                FROM subscriber_lists
                WHERE status = 'unsubscribed'
                AND updated_at >= DATE_TRUNC('month', CURRENT_DATE)
            )
            SELECT 
                s.total_subscribers, s.active_subscribers, s.new_today, s.new_this_week, s.new_this_month,
                c.total_campaigns, c.last_30_days, c.active_campaigns, c.completed_campaigns, 
                c.failed_campaigns, c.total_emails_sent_unique, c.total_emails_sent,
                l.total_lists, l.active_lists, l.avg_list_size, l.max_list_size,
                e.total_opens, e.opens_today, e.opens_this_week, e.opens_this_month, 
                e.avg_opens_per_campaign,
                u.unsubscribes_this_month
            FROM subscriber_stats s
            CROSS JOIN campaign_stats c
            CROSS JOIN list_stats l
            CROSS JOIN engagement_stats e
            CROSS JOIN unsubscribe_stats u
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        // Get peak engagement hour
        let peak_hour = sqlx::query!(
            r#"
            SELECT EXTRACT(HOUR FROM opened_at)::int as hour,
                   COUNT(*) as opens
            FROM email_views
            GROUP BY hour
            ORDER BY opens DESC
            LIMIT 1
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        // Get top countries
        let top_countries = sqlx::query_as!(
            CountryEngagement,
            r#"
            SELECT 
                COALESCE(country, 'Unknown') as "country!",
                COUNT(*) as "total_opens!",
                COUNT(DISTINCT subscriber_id) as "unique_subscribers!",
                CAST(COUNT(DISTINCT subscriber_id) AS float8) / 
                    NULLIF(COUNT(*), 0) * 100.0 as "engagement_rate!"
            FROM email_views
            GROUP BY country
            ORDER BY COUNT(*) DESC
            LIMIT 5
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // Get top cities
        let top_cities = sqlx::query_as!(
            CityEngagement,
            r#"
            SELECT 
                COALESCE(city, 'Unknown') as "city!",
                COALESCE(country, 'Unknown') as "country!",
                COUNT(*) as "total_opens!",
                COUNT(DISTINCT subscriber_id) as "unique_subscribers!",
                CAST(COUNT(DISTINCT subscriber_id) AS float8) / 
                    NULLIF(COUNT(*), 0) * 100.0 as "engagement_rate!"
            FROM email_views
            GROUP BY city, country
            ORDER BY COUNT(*) DESC
            LIMIT 5
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // Calculate growth rate
        let subscriber_growth_rate = if overall.total_subscribers.unwrap_or(0) > 0 {
            (overall.new_this_month.unwrap_or(0) as f64 / overall.total_subscribers.unwrap_or(0) as f64) * 100.0
        } else {
            0.0
        };

        // Calculate global open rate
        let global_open_rate = if overall.total_emails_sent_unique.unwrap_or(0) > 0 {
            (overall.total_opens.unwrap_or(0) as f64 / overall.total_emails_sent_unique.unwrap_or(0) as f64) * 100.0
        } else {
            0.0
        };

        Ok(GlobalStats {
            total_subscribers: overall.total_subscribers.unwrap_or(0),
            active_subscribers: overall.active_subscribers.unwrap_or(0),
            total_campaigns: overall.total_campaigns.unwrap_or(0),
            total_lists: overall.total_lists.unwrap_or(0),
            total_emails_sent: overall.total_emails_sent
                .and_then(|v| v.to_i64())
                .unwrap_or(0),
            total_opens: overall.total_opens.unwrap_or(0),
            global_open_rate,

            campaigns_last_30_days: overall.last_30_days.unwrap_or(0),
            active_campaigns: overall.active_campaigns.unwrap_or(0),
            completed_campaigns: overall.completed_campaigns.unwrap_or(0),
            failed_campaigns: overall.failed_campaigns.unwrap_or(0),

            new_subscribers_today: overall.new_today.unwrap_or(0),
            new_subscribers_this_week: overall.new_this_week.unwrap_or(0),
            new_subscribers_this_month: overall.new_this_month.unwrap_or(0),
            unsubscribes_this_month: overall.unsubscribes_this_month.unwrap_or(0),
            subscriber_growth_rate,

            opens_today: overall.opens_today.unwrap_or(0),
            opens_this_week: overall.opens_this_week.unwrap_or(0),
            opens_this_month: overall.opens_this_month.unwrap_or(0),
            average_opens_per_campaign: overall.avg_opens_per_campaign
                .and_then(|v| v.to_f64())
                .unwrap_or(0.0),

            peak_engagement_hour: peak_hour.hour.unwrap_or(0),
            peak_engagement_day: "Monday".to_string(),

            top_countries,
            top_cities,

            average_list_size: overall.avg_list_size
                .and_then(|v| v.to_f64())
                .unwrap_or(0.0),
            largest_list_size: overall.max_list_size.unwrap_or(0),
            total_active_lists: overall.active_lists.unwrap_or(0),

            average_delivery_time: 0.0,
            bounce_rate: 0.0,
            complaint_rate: 0.0,
        })
    }
} 