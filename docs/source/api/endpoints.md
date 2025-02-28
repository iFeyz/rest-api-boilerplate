# Points d'API

## Email Views

- `GET /api/email_views/{subscriber_id}/{sequence_email_id}/{campaign_id}`
  - Enregistre une ouverture d'email
  - Collecte les données de géolocalisation

## Send Email

- `POST /api/send-email/lists`
  - Envoie des emails à une liste d'abonnés
  - Supporte le tracking d'ouverture 