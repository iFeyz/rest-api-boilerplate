API Endpoints
============

Email Views
----------

GET /api/email_views/{subscriber_id}/{sequence_email_id}/{campaign_id}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Endpoint pour le tracking des ouvertures d'emails.

**Paramètres URL:**

* subscriber_id: ID de l'abonné
* sequence_email_id: ID de l'email dans la séquence
* campaign_id: ID de la campagne

**Exemple de réponse:**

.. code-block:: json

    {
        "id": 1,
        "subscriber_id": 123,
        "sequence_email_id": 456,
        "campaign_id": 789,
        "opened_at": "2025-01-15T11:36:54Z",
        "ip_address": "127.0.0.1",
        "user_agent": "Mozilla/5.0...",
        "country": "France",
        "city": "Paris"
    } 