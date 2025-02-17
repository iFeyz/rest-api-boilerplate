Bienvenue dans la documentation de Mailer API !
===========================================

.. toctree::
   :maxdepth: 2
   :caption: Contenu:

   modules/email_service
   modules/email_views
   api/endpoints

Référence API
=============

.. toctree::
   :maxdepth: 2
   :caption: Documentation API:

   api/email_service
   api/email_views
   api/models

Index et tables
==============

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`

Documentation Mailer API
=======================

.. toctree::
   :maxdepth: 2
   :caption: Contenu:

   api/endpoints
   modules/email_service
   models/email_views

Architecture
-----------

Le système est composé de plusieurs modules:

* **Service Email**: Gestion de l'envoi d'emails
* **Tracking**: Suivi des ouvertures d'emails
* **API REST**: Points d'accès HTTP
* **Base de données**: Stockage PostgreSQL

Base de données
--------------

.. code-block:: sql

   -- Table email_views
   CREATE TABLE email_views (
       id SERIAL PRIMARY KEY,
       subscriber_id INTEGER,
       sequence_email_id INTEGER,
       campaign_id INTEGER,
       opened_at TIMESTAMP WITH TIME ZONE
   );

Index et Tables
--------------

* :ref:`genindex`
* :ref:`search` 