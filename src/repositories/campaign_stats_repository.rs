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

        // Get country stats
        let country_stats = sqlx::query_as!(
            CountryStats,
            r#"
            WITH total_opens AS (
                SELECT COUNT(DISTINCT ev.id) as total
                FROM email_views ev
                JOIN sequence_emails se ON ev.sequence_email_id = se.id
                WHERE se.campaign_id = $1
            )
            SELECT 
                COALESCE(ev.country, 'Unknown') as "country!",
                COUNT(ev.id) as "total_opens!",
                COUNT(DISTINCT ev.subscriber_id) as "unique_opens!",
                CASE 
                    WHEN (SELECT total FROM total_opens) > 0 
                    THEN (CAST(COUNT(ev.id) AS float8) / CAST((SELECT total FROM total_opens) AS float8) * 100.0)
                    ELSE 0.0 
                END as "percentage!: f64"
            FROM email_views ev
            JOIN sequence_emails se ON ev.sequence_email_id = se.id
            WHERE se.campaign_id = $1
            GROUP BY ev.country
            ORDER BY COUNT(ev.id) DESC
            LIMIT 10
            "#,
            campaign_id
        )
        .fetch_all(&self.pool)
        .await?;

        // Get city stats
        let city_stats = sqlx::query_as!(
            CityStats,
            r#"
            WITH total_opens AS (
                SELECT COUNT(DISTINCT ev.id) as total
                FROM email_views ev
                JOIN sequence_emails se ON ev.sequence_email_id = se.id
                WHERE se.campaign_id = $1
            )
            SELECT 
                COALESCE(ev.city, 'Unknown') as "city!",
                COALESCE(ev.country, 'Unknown') as "country!",
                COUNT(ev.id) as "total_opens!",
                COUNT(DISTINCT ev.subscriber_id) as "unique_opens!",
                CASE 
                    WHEN (SELECT total FROM total_opens) > 0 
                    THEN (CAST(COUNT(ev.id) AS float8) / CAST((SELECT total FROM total_opens) AS float8) * 100.0)
                    ELSE 0.0 
                END as "percentage!: f64"
            FROM email_views ev
            JOIN sequence_emails se ON ev.sequence_email_id = se.id
            WHERE se.campaign_id = $1
            GROUP BY ev.city, ev.country
            ORDER BY COUNT(ev.id) DESC
            LIMIT 10
            "#,
            campaign_id
        )
        .fetch_all(&self.pool)
        .await?;

        // Get opens by hour
        let opens_by_hour = sqlx::query_as!(
            TimeStats,
            r#"
            WITH total_opens AS (
                SELECT COUNT(DISTINCT ev.id) as total
                FROM email_views ev
                JOIN sequence_emails se ON ev.sequence_email_id = se.id
                WHERE se.campaign_id = $1
            )
            SELECT 
                EXTRACT(HOUR FROM ev.opened_at)::text as "time_period!",
                COUNT(ev.id) as "total_opens!",
                CASE 
                    WHEN (SELECT total FROM total_opens) > 0 
                    THEN (CAST(COUNT(ev.id) AS float8) / CAST((SELECT total FROM total_opens) AS float8) * 100.0)
                    ELSE 0.0 
                END as "percentage!: f64"
            FROM email_views ev
            JOIN sequence_emails se ON ev.sequence_email_id = se.id
            WHERE se.campaign_id = $1
            GROUP BY EXTRACT(HOUR FROM ev.opened_at)
            ORDER BY EXTRACT(HOUR FROM ev.opened_at)
            "#,
            campaign_id
        )
        .fetch_all(&self.pool)
        .await?;

        // Get opens by day
        let opens_by_day = sqlx::query_as!(
            TimeStats,
            r#"
            WITH total_opens AS (
                SELECT COUNT(DISTINCT ev.id) as total
                FROM email_views ev
                JOIN sequence_emails se ON ev.sequence_email_id = se.id
                WHERE se.campaign_id = $1
            )
            SELECT 
                to_char(ev.opened_at, 'Day') as "time_period!",
                COUNT(ev.id) as "total_opens!",
                CASE 
                    WHEN (SELECT total FROM total_opens) > 0 
                    THEN (CAST(COUNT(ev.id) AS float8) / CAST((SELECT total FROM total_opens) AS float8) * 100.0)
                    ELSE 0.0 
                END as "percentage!: f64"
            FROM email_views ev
            JOIN sequence_emails se ON ev.sequence_email_id = se.id
            WHERE se.campaign_id = $1
            GROUP BY to_char(ev.opened_at, 'Day'), EXTRACT(DOW FROM ev.opened_at)
            ORDER BY EXTRACT(DOW FROM ev.opened_at)
            "#,
            campaign_id
        )
        .fetch_all(&self.pool)
        .await?;

        // Ajouter ces requêtes
        let opened_subscribers = sqlx::query_as!(
            SubscriberOpenInfo,
            r#"
            SELECT 
                s.id as subscriber_id,
                s.email,
                MIN(ev.opened_at) as "first_open!",
                COUNT(ev.id)::bigint as "open_count!"
            FROM subscribers s
            JOIN email_views ev ON s.id = ev.subscriber_id
            WHERE ev.campaign_id = $1
            GROUP BY s.id, s.email
            ORDER BY MIN(ev.opened_at) ASC
            "#,
            campaign_id
        )
        .fetch_all(&self.pool)
        .await?;

        let unopened_subscribers = sqlx::query_as!(
            SubscriberInfo,
            r#"
            SELECT 
                s.id as subscriber_id,
                s.email
            FROM subscribers s
            JOIN campaign_lists cl ON cl.campaign_id = $1
            JOIN subscriber_lists sl ON sl.list_id = cl.list_id AND sl.subscriber_id = s.id
            WHERE NOT EXISTS (
                SELECT 1 
                FROM email_views ev 
                WHERE ev.campaign_id = $1 
                AND ev.subscriber_id = s.id
            )
            AND sl.status = 'confirmed'
            "#,
            campaign_id
        )
        .fetch_all(&self.pool)
        .await?;

        let unopened_count = unopened_subscribers.len() as i64;
        let total_sent = overall_stats.total_sent.unwrap_or(0);
        let unique_opens = overall_stats.unique_opens.unwrap_or(0);
        let total_opens = overall_stats.total_opens.unwrap_or(0);

        let open_rate = if total_sent > 0 {
            (unique_opens as f64 / total_sent as f64) * 100.0
        } else {
            0.0
        };

        Ok(CampaignStats {
            campaign_id,
            campaign_name: campaign.name,
            status: campaign.status,
            start_date: campaign.started_at,
            total_subscribers: overall_stats.total_subscribers.unwrap_or(0),
            total_sent,
            total_opens,
            unique_opens,
            open_rate,
            total_sequence_emails: sequence_stats.len() as i64,
            sequence_stats,
            country_stats,
            city_stats,
            opens_by_hour,
            opens_by_day,
            unopened_count,
            opened_subscribers,
            unopened_subscribers,
        })
    }

    pub async fn get_campaign_detailed_stats(&self, campaign_id: i32) -> Result<CampaignStats, ApiError> {
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

        // Get overall campaign stats
        let base_stats = sqlx::query!(
            r#"
            WITH total_stats AS (
                SELECT 
                    COUNT(DISTINCT sl.subscriber_id) as total_subscribers
                FROM campaign_lists cl
                JOIN subscriber_lists sl ON cl.list_id = sl.list_id
                WHERE cl.campaign_id = $1 AND sl.status = 'confirmed'
            )
            SELECT 
                c.id as campaign_id,
                ts.total_subscribers,
                COUNT(DISTINCT ev.subscriber_id)::bigint as unique_opens,
                COUNT(ev.id)::bigint as total_opens
            FROM campaigns c
            CROSS JOIN total_stats ts
            LEFT JOIN sequence_emails se ON se.campaign_id = c.id
            LEFT JOIN email_views ev ON ev.sequence_email_id = se.id
            WHERE c.id = $1
            GROUP BY c.id, ts.total_subscribers
            "#,
            campaign_id
        )
        .fetch_one(&self.pool)
        .await?;

        // Get sequence email stats
        let sequence_stats = sqlx::query_as!(
            SequenceEmailStats,
            r#"
            WITH subscriber_count AS (
                SELECT COUNT(DISTINCT sl.subscriber_id) as total_subscribers
                FROM campaign_lists cl
                JOIN subscriber_lists sl ON cl.list_id = sl.list_id
                WHERE cl.campaign_id = $1 AND sl.status = 'confirmed'
            )
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
            "#,
            campaign_id
        )
        .fetch_all(&self.pool)
        .await?;

        // Get subscribers who opened any email in the sequence
        let opened_subscribers = sqlx::query_as!(
            SubscriberOpenInfo,
            r#"
            SELECT 
                s.id as subscriber_id,
                s.email,
                MIN(ev.opened_at) as "first_open!",
                COUNT(ev.id)::bigint as "open_count!"
            FROM subscribers s
            JOIN email_views ev ON s.id = ev.subscriber_id
            JOIN sequence_emails se ON ev.sequence_email_id = se.id
            WHERE se.campaign_id = $1
            GROUP BY s.id, s.email
            ORDER BY MIN(ev.opened_at) ASC
            "#,
            campaign_id
        )
        .fetch_all(&self.pool)
        .await?;

        // Get subscribers who haven't opened any email
        let unopened_subscribers = sqlx::query_as!(
            SubscriberInfo,
            r#"
            SELECT 
                s.id as subscriber_id,
                s.email
            FROM subscribers s
            JOIN campaign_lists cl ON cl.campaign_id = $1
            JOIN subscriber_lists sl ON sl.list_id = cl.list_id AND sl.subscriber_id = s.id
            WHERE sl.status = 'confirmed'
            AND NOT EXISTS (
                SELECT 1 
                FROM email_views ev 
                JOIN sequence_emails se ON ev.sequence_email_id = se.id
                WHERE se.campaign_id = $1 
                AND ev.subscriber_id = s.id
            )
            "#,
            campaign_id
        )
        .fetch_all(&self.pool)
        .await?;

        let total_subscribers = base_stats.total_subscribers.unwrap_or(0);
        let unopened_count = unopened_subscribers.len() as i64;
        let unique_opens = base_stats.unique_opens.unwrap_or(0);
        let total_opens = base_stats.total_opens.unwrap_or(0);

        let open_rate = if total_subscribers > 0 {
            (unique_opens as f64 / total_subscribers as f64) * 100.0
        } else {
            0.0
        };

        Ok(CampaignStats {
            campaign_id: base_stats.campaign_id,
            campaign_name: campaign.name,
            status: campaign.status,
            start_date: campaign.started_at,
            total_subscribers,
            total_sent: total_subscribers, // Total subscribers is our total sent for sequence campaigns
            total_opens,
            unique_opens,
            open_rate,
            unopened_count,
            total_sequence_emails: sequence_stats.len() as i64,
            sequence_stats,
            country_stats: Vec::new(),
            city_stats: Vec::new(),
            opens_by_hour: Vec::new(),
            opens_by_day: Vec::new(),
            opened_subscribers,
            unopened_subscribers,
        })
    }

    pub async fn get_sequence_email_stats(
        &self,
        campaign_id: i32,
        sequence_id: i32
    ) -> Result<SequenceEmailDetailedStats, ApiError> {
        // Get sequence email info and basic stats
        let base_stats = sqlx::query!(
            r#"
            WITH subscriber_count AS (
                SELECT COUNT(DISTINCT sl.subscriber_id) as total_subscribers
                FROM campaign_lists cl
                JOIN subscriber_lists sl ON cl.list_id = sl.list_id
                WHERE cl.campaign_id = $1 AND sl.status = 'confirmed'
            )
            SELECT 
                se.id as "sequence_email_id!",
                se.campaign_id as "campaign_id!",
                se.position as "position!",
                se.subject as "subject!",
                se.status as "status!: String",
                se.send_at as sent_at,
                (SELECT total_subscribers FROM subscriber_count) as "total_subscribers!",
                COUNT(ev.id) as "total_opens!",
                COUNT(DISTINCT ev.subscriber_id) as "unique_opens!"
            FROM sequence_emails se
            LEFT JOIN email_views ev ON se.id = ev.sequence_email_id
            WHERE se.campaign_id = $1 AND se.id = $2
            GROUP BY se.id, se.campaign_id, se.position, se.subject, se.status, se.send_at
            "#,
            campaign_id,
            sequence_id
        )
        .fetch_one(&self.pool)
        .await?;

        // Get subscribers who opened this specific email
        let opened_subscribers = sqlx::query_as!(
            SubscriberOpenInfo,
            r#"
            SELECT 
                s.id as subscriber_id,
                s.email,
                MIN(ev.opened_at) as "first_open!",
                COUNT(ev.id)::bigint as "open_count!"
            FROM subscribers s
            JOIN email_views ev ON s.id = ev.subscriber_id
            WHERE ev.sequence_email_id = $1
            GROUP BY s.id, s.email
            ORDER BY MIN(ev.opened_at) ASC
            "#,
            sequence_id
        )
        .fetch_all(&self.pool)
        .await?;

        // Get subscribers who haven't opened this specific email
        let unopened_subscribers = sqlx::query_as!(
            SubscriberInfo,
            r#"
            SELECT 
                s.id as subscriber_id,
                s.email
            FROM subscribers s
            JOIN campaign_lists cl ON cl.campaign_id = $1
            JOIN subscriber_lists sl ON sl.list_id = cl.list_id AND sl.subscriber_id = s.id
            WHERE sl.status = 'confirmed'
            AND NOT EXISTS (
                SELECT 1 
                FROM email_views ev 
                WHERE ev.sequence_email_id = $2
                AND ev.subscriber_id = s.id
            )
            "#,
            campaign_id,
            sequence_id
        )
        .fetch_all(&self.pool)
        .await?;

        let open_rate = if base_stats.total_subscribers > 0 {
            (base_stats.unique_opens as f64 / base_stats.total_subscribers as f64) * 100.0
        } else {
            0.0
        };

        Ok(SequenceEmailDetailedStats {
            sequence_email_id: base_stats.sequence_email_id,
            campaign_id: base_stats.campaign_id,
            subject: base_stats.subject,
            position: base_stats.position,
            status: base_stats.status,
            sent_at: base_stats.sent_at,
            total_subscribers: base_stats.total_subscribers,
            total_opens: base_stats.total_opens,
            unique_opens: base_stats.unique_opens,
            open_rate,
            opened_subscribers,
            unopened_subscribers,
        })
    }
}

// ... rest of the implementation ... 