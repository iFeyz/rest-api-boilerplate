Modèles Email Views
==================

EmailView
--------

.. code-block:: rust

    pub struct EmailView {
        pub id: i32,
        pub campaign_id: Option<i32>,
        pub sequence_email_id: Option<i32>,
        pub subscriber_id: Option<i32>,
        pub opened_at: Option<DateTime<Utc>>,
        pub ip_address: Option<String>,
        pub user_agent: Option<String>,
        pub country: Option<String>,
        pub city: Option<String>,
        pub region: Option<String>,
        pub metadata: JsonValue,
        pub created_at: Option<DateTime<Utc>>,
    }

Champs
~~~~~~

* id: Identifiant unique de la vue
* campaign_id: ID de la campagne associée
* sequence_email_id: ID de l'email dans la séquence
* subscriber_id: ID de l'abonné
* opened_at: Date et heure d'ouverture
* ip_address: Adresse IP du lecteur
* user_agent: User agent du navigateur
* country: Pays détecté
* city: Ville détectée
* region: Région détectée
* metadata: Données additionnelles au format JSON
* created_at: Date de création de l'enregistrement 