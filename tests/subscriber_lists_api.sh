#!/bin/bash

# Configuration
API_URL="http://localhost:8080"
ENDPOINT="/api/subscriber_lists"
API_KEY="OlH2V4j/OMfBnxfUvsrjoiD9xcI+/ihMv1go8/hf2HI="

echo "Testing Subscriber Lists API..."

# CREATE - Create a new subscription
echo -e "\n1. Creating a new subscription..."
curl -X POST "${API_URL}${ENDPOINT}" \
-H 'Content-Type: application/json' \
-H "X-API-Key: ${API_KEY}" \
-d '{
    "subscriber_id": 1,
    "list_id": 1,
    "status": "unconfirmed",
    "meta": {
        "source": "website",
        "ip": "127.0.0.1"
    }
}'

# READ - Get a specific subscription
echo -e "\n\n2. Getting a specific subscription..."
curl -X GET "${API_URL}${ENDPOINT}?subscriber_id=1&list_id=1" \
-H "X-API-Key: ${API_KEY}"

# READ ALL - List subscriptions with different filters
echo -e "\n\n3. Listing all subscriptions with pagination and filters..."
curl -X GET "${API_URL}${ENDPOINT}/all?page=1&per_page=10&order_by=created_at&order=DESC&status=confirmed" \
-H "X-API-Key: ${API_KEY}"

echo -e "\n\n4. Listing subscriptions for a specific subscriber..."
curl -X GET "${API_URL}${ENDPOINT}/all?subscriber_id=1" \
-H "X-API-Key: ${API_KEY}"

echo -e "\n\n5. Listing subscriptions for a specific list..."
curl -X GET "${API_URL}${ENDPOINT}/all?list_id=1" \
-H "X-API-Key: ${API_KEY}"

# UPDATE - Update a subscription
echo -e "\n\n6. Updating a subscription..."
curl -X PUT "${API_URL}${ENDPOINT}/1/1" \
-H 'Content-Type: application/json' \
-H "X-API-Key: ${API_KEY}" \
-d '{
    "status": "confirmed",
    "meta": {
        "confirmed_at": "2024-01-13T16:00:00Z",
        "confirmation_ip": "127.0.0.1"
    }
}'

# DELETE - Delete a subscription
echo -e "\n\n7. Deleting a subscription..."
curl -X DELETE "${API_URL}${ENDPOINT}/1/1" \
-H "X-API-Key: ${API_KEY}"

echo -e "\n\nAPI testing completed!" 