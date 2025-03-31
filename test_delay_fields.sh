#!/bin/bash
set -e

# Configuration
API_URL="http://localhost:8081"
API_KEY="OlH2V4j/OMfBnxfUvsrjoiD9xcI+/ihMv1go8/hf2HI="

# Colors for terminal output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Testing Sequence Email Delay Fields${NC}"

# Create a test sequence email with delay fields
echo -e "${YELLOW}Creating a test sequence email with delay fields...${NC}"
RESPONSE=$(curl -s -X POST "$API_URL/api/sequence-emails" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{
    "campaign_id": 1,
    "position": 1,
    "subject": "Test Email with Delay",
    "body": "<h1>Test Email</h1><p>This is a test email with delay fields.</p>",
    "content_type": "Html",
    "is_active": true,
    "delay_type": "after_join",
    "delay_value": 30,
    "delay_unit": "minutes",
    "metadata": {},
    "status": "Draft"
  }')

# Parse the response
echo -e "Response: $RESPONSE"

# Try to get the sequence email to check if the delay fields were saved
EMAIL_ID=$(echo $RESPONSE | grep -o '"id":[0-9]*' | cut -d':' -f2)

if [ -z "$EMAIL_ID" ]; then
  echo -e "${RED}Failed to get email ID${NC}"
  exit 1
fi

echo -e "${YELLOW}Fetching sequence email with ID: $EMAIL_ID...${NC}"
GET_RESPONSE=$(curl -s -X GET "$API_URL/api/sequence-emails/$EMAIL_ID" \
  -H "X-API-Key: $API_KEY")

echo -e "GET Response: $GET_RESPONSE"

# Check if the delay fields are present in the response
DELAY_TYPE=$(echo $GET_RESPONSE | grep -o '"delay_type":"[^"]*"' || echo "")
DELAY_VALUE=$(echo $GET_RESPONSE | grep -o '"delay_value":[0-9]*' || echo "")
DELAY_UNIT=$(echo $GET_RESPONSE | grep -o '"delay_unit":"[^"]*"' || echo "")

if [ -z "$DELAY_TYPE" ] || [ -z "$DELAY_VALUE" ] || [ -z "$DELAY_UNIT" ]; then
  echo -e "${RED}Delay fields not found in response!${NC}"
else
  echo -e "${GREEN}Delay fields successfully saved:${NC}"
  echo -e "  $DELAY_TYPE"
  echo -e "  $DELAY_VALUE"
  echo -e "  $DELAY_UNIT"
fi

# Cleanup
echo -e "${YELLOW}Cleaning up - Deleting test sequence email...${NC}"
DELETE_RESPONSE=$(curl -s -X DELETE "$API_URL/api/sequence-emails/$EMAIL_ID" \
  -H "X-API-Key: $API_KEY")

echo -e "${GREEN}Test completed.${NC}" 