#!/bin/bash
set -e

# Mettre à jour le système
apt-get update && apt-get upgrade -y

# Installer Docker et Docker Compose si nécessaire
if ! command -v docker &> /dev/null; then
    curl -fsSL https://get.docker.com -o get-docker.sh
    sh get-docker.sh
fi

if ! command -v docker-compose &> /dev/null; then
    curl -L "https://github.com/docker/compose/releases/download/v2.24.5/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    chmod +x /usr/local/bin/docker-compose
fi

# Créer les répertoires nécessaires
mkdir -p certbot/conf certbot/www

# Démarrer les conteneurs
docker-compose -f docker-compose.prod.yml up -d

# Obtenir le certificat SSL initial
docker-compose -f docker-compose.prod.yml run --rm certbot certonly --webroot --webroot-path /var/www/certbot --email your-email@domain.com -d your-domain.com --agree-tos --no-eff-email

# Redémarrer nginx pour appliquer le certificat
docker-compose -f docker-compose.prod.yml restart nginx 