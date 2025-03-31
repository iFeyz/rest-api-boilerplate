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

echo -e "${YELLOW}Starting Opt-in Email Campaign Test${NC}"

# Step 1: Create a list
echo -e "${YELLOW}Creating a new subscriber list...${NC}"
LIST_RESPONSE=$(curl -s -X POST "$API_URL/api/lists" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{
    "name": "Opt-in Test List",
    "type": "Public",
    "optin": "Single",
    "tags": ["test", "optin"],
    "description": "Test list for opt-in campaign with sequence emails"
  }')

LIST_ID=$(echo $LIST_RESPONSE | grep -o '"id":[0-9]*' | cut -d':' -f2)

if [ -z "$LIST_ID" ]; then
  echo -e "${RED}Failed to create list${NC}"
  echo $LIST_RESPONSE
  exit 1
fi

echo -e "${GREEN}Successfully created list with ID: $LIST_ID${NC}"

# Step 2: Create an opt-in campaign
echo -e "${YELLOW}Creating an opt-in campaign...${NC}"
CAMPAIGN_RESPONSE=$(curl -s -X POST "$API_URL/api/campaigns" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{
    "name": "Test Opt-in Sequence",
    "subject": "Welcome to Our Newsletter",
    "from_email": "test@example.com",
    "campaign_type": "Optin",
    "status": "Draft",
    "messenger": "smtp"
  }')

CAMPAIGN_ID=$(echo $CAMPAIGN_RESPONSE | grep -o '"id":[0-9]*' | cut -d':' -f2)

if [ -z "$CAMPAIGN_ID" ]; then
  echo -e "${RED}Failed to create campaign${NC}"
  echo $CAMPAIGN_RESPONSE
  exit 1
fi

echo -e "${GREEN}Successfully created campaign with ID: $CAMPAIGN_ID${NC}"

# Step 3: Associate the list with the campaign
echo -e "${YELLOW}Associating list with campaign...${NC}"
ASSOC_RESPONSE=$(curl -s -X POST "$API_URL/api/campaign_lists" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d "{
    \"campaign_id\": $CAMPAIGN_ID,
    \"list_id\": $LIST_ID,
    \"list_name\": \"Opt-in Test List\"
  }")

echo -e "${GREEN}Successfully associated list with campaign${NC}"

# Step 4: Create sequence emails
# Email 1 - Sent immediately when user subscribes
echo -e "${YELLOW}Creating first sequence email (immediate)...${NC}"
EMAIL1_RESPONSE=$(curl -s -X POST "$API_URL/api/sequence-emails" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d "{
    \"campaign_id\": $CAMPAIGN_ID,
    \"position\": 1,
    \"subject\": \"Welcome to Our Newsletter\",
    \"body\": \"<h1>Welcome!</h1><p>Thank you for subscribing to our newsletter. This is the first email in our sequence.</p>\",
    \"content_type\": \"Html\",
    \"is_active\": true,
    \"delay_type\": \"after_join\",
    \"delay_value\": 0,
    \"delay_unit\": \"minutes\",
    \"metadata\": {},
    \"status\": \"Draft\"
  }")

EMAIL1_ID=$(echo $EMAIL1_RESPONSE | grep -o '"id":[0-9]*' | cut -d':' -f2)

if [ -z "$EMAIL1_ID" ]; then
  echo -e "${RED}Failed to create first email${NC}"
  echo $EMAIL1_RESPONSE
  exit 1
fi

echo -e "${GREEN}Successfully created first email with ID: $EMAIL1_ID${NC}"

# Email 2 - Sent 2 minutes after the first email
echo -e "${YELLOW}Creating second sequence email (2 minutes delay)...${NC}"
EMAIL2_RESPONSE=$(curl -s -X POST "$API_URL/api/sequence-emails" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d "{
    \"campaign_id\": $CAMPAIGN_ID,
    \"position\": 2,
    \"subject\": \"Getting Started with Our Service\",
    \"body\": \"<h1>Getting Started</h1><p>Here are some tips to get started with our service. This is the second email in our sequence.</p>\",
    \"content_type\": \"Html\",
    \"is_active\": true,
    \"delay_type\": \"after_previous\",
    \"delay_value\": 2,
    \"delay_unit\": \"minutes\",
    \"metadata\": {},
    \"status\": \"Draft\"
  }")

EMAIL2_ID=$(echo $EMAIL2_RESPONSE | grep -o '"id":[0-9]*' | cut -d':' -f2)

if [ -z "$EMAIL2_ID" ]; then
  echo -e "${RED}Failed to create second email${NC}"
  echo $EMAIL2_RESPONSE
  exit 1
fi

echo -e "${GREEN}Successfully created second email with ID: $EMAIL2_ID${NC}"

# Email 3 - Sent 3 minutes after the second email
echo -e "${YELLOW}Creating third sequence email (3 minutes delay)...${NC}"
EMAIL3_RESPONSE=$(curl -s -X POST "$API_URL/api/sequence-emails" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d "{
    \"campaign_id\": $CAMPAIGN_ID,
    \"position\": 3,
    \"subject\": \"Advanced Features\",
    \"body\": \"<h1>Advanced Features</h1><p>Discover our advanced features. This is the third and final email in our sequence.</p>\",
    \"content_type\": \"Html\",
    \"is_active\": true,
    \"delay_type\": \"after_previous\",
    \"delay_value\": 3,
    \"delay_unit\": \"minutes\",
    \"metadata\": {},
    \"status\": \"Draft\"
  }")

EMAIL3_ID=$(echo $EMAIL3_RESPONSE | grep -o '"id":[0-9]*' | cut -d':' -f2)

if [ -z "$EMAIL3_ID" ]; then
  echo -e "${RED}Failed to create third email${NC}"
  echo $EMAIL3_RESPONSE
  exit 1
fi

echo -e "${GREEN}Successfully created third email with ID: $EMAIL3_ID${NC}"

# Step 5: Update campaign status to "Running"
echo -e "${YELLOW}Setting campaign to 'Running' status...${NC}"
UPDATE_RESPONSE=$(curl -s -X PUT "$API_URL/api/campaigns/$CAMPAIGN_ID" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $API_KEY" \
  -d '{
    "status": "Running"
  }')

echo -e "${GREEN}Campaign is now active${NC}"

# Step 6: Add a test subscriber to the list using the subscriber-sequence endpoint
echo -e "${YELLOW}Adding a test subscriber to trigger the sequence...${NC}"
TEST_EMAIL="ajochum.pro@gmail.com"

SUBSCRIBE_RESPONSE=$(curl -s -X POST "$API_URL/api/subscriber-sequence/$TEST_EMAIL/lists/$LIST_ID" \
  -H "X-API-Key: $API_KEY")

echo -e "${GREEN}Test subscriber added! Sequence emails should now be triggered.${NC}"
echo -e "Subscription response: $SUBSCRIBE_RESPONSE"

# Summary
echo
echo -e "${GREEN}=========== TEST SUMMARY ===========${NC}"
echo -e "List ID: ${YELLOW}$LIST_ID${NC}"
echo -e "Campaign ID: ${YELLOW}$CAMPAIGN_ID${NC}"
echo -e "Sequence Emails:"
echo -e "  Email 1 (immediate): ${YELLOW}$EMAIL1_ID${NC}"
echo -e "  Email 2 (2 min delay): ${YELLOW}$EMAIL2_ID${NC}"
echo -e "  Email 3 (3 min delay): ${YELLOW}$EMAIL3_ID${NC}"
echo -e "Test Subscriber: ${YELLOW}$TEST_EMAIL${NC}"
echo
echo -e "${GREEN}The subscriber should receive:${NC}"
echo -e "- First email immediately"
echo -e "- Second email 2 minutes after the first email"
echo -e "- Third email 3 minutes after the second email"
echo
echo -e "${YELLOW}Check the logs to confirm delivery${NC}" 