use sqlx::PgPool;
use chrono::{DateTime, Utc, Duration};
use crate::models::subscriber_sequence_progress::{
    SubscriberSequenceProgress, 
    CreateSequenceProgressDto, 
    UpdateSequenceProgressDto,
    DelayType,
    DelayUnit
};
use crate::repositories::subscriber_sequence_progress_repository::SubscriberSequenceProgressRepository;
use crate::email_service::EmailService;
use crate::error::ApiError;
use crate::models::sequence_email::{SequenceEmail, SequenceEmailStatus};
use crate::repositories::sequence_email_repository::SequenceEmailRepository;
use crate::repositories::campaign_repository::CampaignRepository;
use crate::repositories::campaign_list_repository::CampaignListRepository;
use crate::models::campaign::CampaignType;

pub struct SequenceOptinService {
    sequence_progress_repo: SubscriberSequenceProgressRepository,
    sequence_email_repo: SequenceEmailRepository,
    campaign_repo: CampaignRepository,
    campaign_list_repo: CampaignListRepository,
    email_service: EmailService,
    pool: PgPool,
}

impl SequenceOptinService {
    pub fn new(
        pool: PgPool,
        email_service: EmailService,
    ) -> Self {
        Self {
            sequence_progress_repo: SubscriberSequenceProgressRepository::new(pool.clone()),
            sequence_email_repo: SequenceEmailRepository::new(pool.clone()),
            campaign_repo: CampaignRepository::new(pool.clone()),
            campaign_list_repo: CampaignListRepository::new(pool.clone()),
            email_service,
            pool,
        }
    }

    // Initialiser les séquences pour un abonné ajouté à une liste
    pub async fn initialize_sequences_for_subscriber(
        &self,
        subscriber_id: i32,
        list_id: i32
    ) -> Result<Vec<SubscriberSequenceProgress>, ApiError> {
        // 1. Trouver toutes les campagnes opt-in associées à cette liste
        let optin_campaigns = self.find_optin_campaigns_for_list(list_id).await?;
        let mut results = Vec::new();

        // 2. Pour chaque campagne, initialiser une séquence
        for campaign in optin_campaigns {
            // Vérifier si une progression existe déjà pour cet abonné et cette campagne
            if let Ok(Some(_)) = self.sequence_progress_repo
                .find_by_subscriber_and_campaign(subscriber_id, campaign.id).await {
                    continue; // Progression existe déjà, passer à la campagne suivante
            }

            // Créer une nouvelle progression
            let progress_dto = CreateSequenceProgressDto {
                subscriber_id,
                campaign_id: campaign.id,
                list_id,
            };

            let progress = self.sequence_progress_repo.create(progress_dto).await?;
            
            // Calculer quand envoyer le premier email
            let first_email = self.get_first_sequence_email(campaign.id).await?;
            
            if let Some(email) = first_email {
                let next_send_time = self.calculate_next_send_time(
                    &progress, 
                    &email,
                    true // Premier email
                ).await?;
                
                // Mettre à jour la progression avec la date du prochain envoi et la position
                let update_dto = UpdateSequenceProgressDto {
                    current_position: Some(email.position), // Use the exact position from the email
                    last_email_sent_at: None,
                    next_email_scheduled_at: Some(next_send_time),
                    completed: None,
                };
                
                tracing::debug!("Initializing sequence with position {} for campaign {}, subscriber {}", 
                                email.position, campaign.id, subscriber_id);
                
                let updated_progress = self.sequence_progress_repo
                    .update(progress.id, update_dto).await?;
                
                results.push(updated_progress);
            }
        }

        Ok(results)
    }

    // Trouver les campagnes de type opt-in pour une liste
    async fn find_optin_campaigns_for_list(&self, list_id: i32) -> Result<Vec<crate::models::campaign::Campaign>, ApiError> {
        let campaign_lists = self.campaign_list_repo.get_campaign_lists_by_list_id(list_id).await?;
        let mut optin_campaigns = Vec::new();

        for campaign_list in campaign_lists {
            if let Ok(Some(campaign)) = self.campaign_repo.find_by_id(campaign_list.campaign_id).await {
                if campaign.campaign_type == CampaignType::Optin {
                    optin_campaigns.push(campaign);
                }
            }
        }

        Ok(optin_campaigns)
    }

    // Obtenir le premier email de la séquence
    async fn get_first_sequence_email(&self, campaign_id: i32) -> Result<Option<SequenceEmail>, ApiError> {
        let sequence_emails = self.sequence_email_repo
            .find_by_campaign_id(campaign_id).await?;
        
        let mut first_email = None;
        let mut min_position = i32::MAX;
        
        for email in sequence_emails {
            if email.position < min_position && email.is_active {
                min_position = email.position;
                first_email = Some(email);
            }
        }
        
        Ok(first_email)
    }

    // Obtenir le prochain email de la séquence
    async fn get_next_sequence_email(
        &self, 
        campaign_id: i32, 
        current_position: i32
    ) -> Result<Option<SequenceEmail>, ApiError> {
        let sequence_emails = self.sequence_email_repo
            .find_by_campaign_id(campaign_id).await?;
        
        let mut next_email = None;
        let mut min_position = i32::MAX;
        
        for email in sequence_emails {
            if email.position > current_position && 
               email.position < min_position && 
               email.is_active {
                min_position = email.position;
                next_email = Some(email);
            }
        }
        
        Ok(next_email)
    }

    // Calculer quand envoyer le prochain email
    async fn calculate_next_send_time(
        &self,
        progress: &SubscriberSequenceProgress,
        sequence_email: &SequenceEmail,
        is_first_email: bool
    ) -> Result<DateTime<Utc>, ApiError> {
        let now = Utc::now();
        
        // Obtenir le type de délai, la valeur et l'unité
        let delay_type_str = sqlx::query_scalar!(
            r#"SELECT delay_type FROM sequence_emails WHERE id = $1"#,
            sequence_email.id
        )
        .fetch_optional(&self.pool)
        .await?
        .unwrap_or_else(|| "absolute".to_string());
        
        let delay_value = sqlx::query_scalar!(
            r#"SELECT delay_value FROM sequence_emails WHERE id = $1"#,
            sequence_email.id
        )
        .fetch_optional(&self.pool)
        .await?
        .unwrap_or(Some(0))
        .unwrap_or(0);
        
        let delay_unit_str = sqlx::query_scalar!(
            r#"SELECT delay_unit FROM sequence_emails WHERE id = $1"#,
            sequence_email.id
        )
        .fetch_optional(&self.pool)
        .await?
        .unwrap_or_else(|| Some("minutes".to_string()))
        .unwrap_or_else(|| "minutes".to_string());
        
        // Convertir les valeurs en types enum
        let delay_type = match delay_type_str.as_str() {
            "after_join" => DelayType::AfterJoin,
            "after_previous" => DelayType::AfterPrevious,
            _ => DelayType::Absolute,
        };
        
        let delay_unit = match delay_unit_str.as_str() {
            "hours" => DelayUnit::Hours,
            "days" => DelayUnit::Days,
            _ => DelayUnit::Minutes,
        };
        
        // Si c'est une date absolue (comportement actuel), utiliser send_at
        if delay_type == DelayType::Absolute {
            return Ok(sequence_email.send_at.unwrap_or(now));
        }
        
        // Calculer le délai en fonction de l'unité
        let duration = match delay_unit {
            DelayUnit::Minutes => Duration::minutes(delay_value as i64),
            DelayUnit::Hours => Duration::hours(delay_value as i64),
            DelayUnit::Days => Duration::days(delay_value as i64),
        };
        
        // Calculer la date d'envoi en fonction du type de délai
        let next_send_time = match delay_type {
            DelayType::AfterJoin => {
                // Délai après ajout à la liste
                progress.joined_at + duration
            },
            DelayType::AfterPrevious => {
                // Délai après l'email précédent
                if is_first_email {
                    // Si c'est le premier email et qu'il a un délai after_previous,
                    // on utilise la date d'ajout comme référence
                    progress.joined_at + duration
                } else {
                    // Sinon on utilise la date du dernier email envoyé
                    progress.last_email_sent_at.unwrap_or(now) + duration
                }
            },
            DelayType::Absolute => sequence_email.send_at.unwrap_or(now), // Cas déjà traité plus haut
        };
        
        // Si la date calculée est dans le passé, utiliser maintenant
        if next_send_time < now {
            Ok(now)
        } else {
            Ok(next_send_time)
        }
    }

    // Traiter les envois de séquence en attente
    pub async fn process_pending_sequence_emails(&self) -> Result<i32, ApiError> {
        // Trouver les progressions qui ont des emails prêts à être envoyés
        let pending_progressions = self.sequence_progress_repo.find_pending_sends().await?;
        let mut sent_count = 0;
        
        tracing::info!("Found {} pending sequence emails to process", pending_progressions.len());
        
        for progress in pending_progressions {
            // Obtenir l'email de séquence correspondant à la position actuelle
            let campaign_id = progress.campaign_id;
            let position = progress.current_position;
            
            tracing::info!("Processing sequence for campaign_id: {}, position: {}, subscriber_id: {}", 
                campaign_id, position, progress.subscriber_id);
            
            let sequence_emails = self.sequence_email_repo.find_by_campaign_id(campaign_id).await?;
            tracing::debug!("Found {} sequence emails for campaign_id {}", sequence_emails.len(), campaign_id);
            
            for email in &sequence_emails {
                tracing::debug!("Email id: {}, position: {}, is_active: {}", email.id, email.position, email.is_active);
            }
            
            let current_email = sequence_emails.iter()
                .find(|e| e.position == position && e.is_active);
            
            if let Some(email) = current_email {
                tracing::info!("Found email to send: id={}, subject='{}'", email.id, email.subject);
                
                // Envoyer l'email
                let subscriber = sqlx::query!(
                    r#"SELECT email FROM subscribers WHERE id = $1"#,
                    progress.subscriber_id
                )
                .fetch_one(&self.pool)
                .await?;
                
                tracing::info!("Sending email to subscriber: {}", subscriber.email);
                
                // Ajouter le tracking à l'email
                let tracked_body = self.email_service.add_tracking_to_email(
                    &email.body,
                    campaign_id,
                    email.id,
                    progress.subscriber_id
                );
                
                // Envoyer l'email
                match self.email_service.send_email(
                    &subscriber.email,
                    &email.subject,
                    &tracked_body
                ).await {
                    Ok(_) => {
                        tracing::info!("Email sent successfully!");
                        let now = Utc::now();
                        
                        // Mettre à jour la progression
                        let next_email = self.get_next_sequence_email(campaign_id, position).await?;
                        
                        // Current position should be set to the next email's position, not incremented
                        let next_position = if let Some(next) = &next_email {
                            next.position
                        } else {
                            position + 1 // If there's no next email, increment to mark as "past the last email"
                        };
                        
                        tracing::debug!("Updating position from {} to {}", position, next_position);
                            
                        let mut update_dto = UpdateSequenceProgressDto {
                            current_position: Some(next_position),
                            last_email_sent_at: Some(now),
                            next_email_scheduled_at: None,
                            completed: None,
                        };
                        
                        // S'il y a un prochain email, calculer quand l'envoyer
                        if let Some(next) = next_email {
                            let next_send_time = self.calculate_next_send_time(
                                &progress, 
                                &next,
                                false // Ce n'est pas le premier email
                            ).await?;
                            
                            tracing::info!("Scheduling next email (position {}) at: {}", next.position, next_send_time);
                            update_dto.next_email_scheduled_at = Some(next_send_time);
                        } else {
                            // Pas de prochain email, marquer comme terminé
                            tracing::info!("Sequence completed for subscriber_id: {}", progress.subscriber_id);
                            update_dto.completed = Some(true);
                        }
                        
                        // Mettre à jour la progression
                        let updated_progress = self.sequence_progress_repo.update(progress.id, update_dto).await?;
                        tracing::debug!("Progress updated, new position: {}", updated_progress.current_position);
                        
                        sent_count += 1;
                    },
                    Err(e) => {
                        tracing::error!("Failed to send sequence email: {}", e);
                        // On pourrait implémenter des tentatives supplémentaires ici
                    }
                }
            } else {
                tracing::warn!("No active email found for campaign_id: {}, position: {}. Available positions: {}", 
                    campaign_id, position, 
                    sequence_emails.iter()
                        .filter(|e| e.is_active)
                        .map(|e| e.position.to_string())
                        .collect::<Vec<String>>()
                        .join(", "));
            }
        }
        
        tracing::info!("Processed {} sequence emails", sent_count);
        
        Ok(sent_count)
    }
}