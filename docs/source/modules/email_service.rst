Service Email
============

Vue d'ensemble
-------------

Le service email gère l'envoi d'emails avec suivi d'ouverture.

Fonctionnalités
--------------

* Envoi d'emails via SMTP
* Suivi des ouvertures d'emails
* Gestion des campagnes
* Envoi en masse

Méthodes principales
------------------

create_tracking_pixel
~~~~~~~~~~~~~~~~~~~

.. code-block:: rust

    pub fn create_tracking_pixel(
        &self,
        campaign_id: i32,
        sequence_email_id: i32,
        subscriber_id: i32,
    ) -> String

Génère un pixel de tracking pour suivre l'ouverture des emails. 