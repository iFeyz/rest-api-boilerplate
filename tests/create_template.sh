#!/bin/bash

# Load environment variables from .env file
if [ -f .env ]; then
    export $(cat .env | grep -v '#' | awk '/=/ {print $1}')
fi

# API endpoint
API_URL="http://localhost:8080/api/templates"

# Create template
echo "Creating new template..."
curl -X POST $API_URL \
  -H "Content-Type: application/json" \
  -H "x-api-key: $API_KEY" \
  -d '{
    "name": "Test Template",
    "template_type": "campaign",
    "subject": "Test Subject",
    "body": "Test Body Content",
    "is_default": false
  }'

echo -e "\n" 