use crate::monitoring::EmailMetrics;

// Dans votre fonction d'envoi d'email
pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
    let email_metrics = self.email_metrics.clone();
    
    match self.client.send_email(to, subject, body).await {
        Ok(_) => {
            email_metrics.record_sent_email();
            Ok(())
        },
        Err(e) => {
            email_metrics.record_failed_email();
            Err(e.into())
        }
    }
} 