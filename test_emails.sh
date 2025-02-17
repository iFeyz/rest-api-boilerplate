#!/bin/bash

# Base URL
BASE_URL="http://localhost:8080"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "Testing Email API Endpoints"
echo "=========================="

# Test 1: Send single email
echo -e "\n${GREEN}Test 1: Sending single email${NC}"
curl -X POST "$BASE_URL/email/send" \
  -H "Content-Type: application/json" \
  -d '{
    "to": "test@example.com",
    "subject": "Test Single Email",
    "body": "<h1>Hello</h1><p>This is a test email</p>"
  }'

# Test 2: Send bulk emails
echo -e "\n\n${GREEN}Test 2: Sending bulk emails${NC}"
curl -X POST "$BASE_URL/email/send-bulk" \
  -H "Content-Type: application/json" \
  -d '{
    "emails": [
      {
        "to": "user1@example.com",
        "subject": "Bulk Test 1",
        "body": "<h1>Hello User 1</h1>"
      },
      {
        "to": "user2@example.com",
        "subject": "Bulk Test 2",
        "body": "<h1>Hello User 2</h1>"
      }
    ]
  }'

# Test 3: Send to lists
echo -e "\n\n${GREEN}Test 3: Sending to lists${NC}"
curl -X POST "$BASE_URL/email/send-to-lists" \
  -H "Content-Type: application/json" \
  -d '{
    "list_ids": [1, 2],
    "subject": "Newsletter Test",
    "body": "<h1>Hello Subscribers!</h1><p>This is a test newsletter.</p>"
  }'

# Test 4: Send to invalid list
echo -e "\n\n${GREEN}Test 4: Testing error handling - Invalid list ID${NC}"
curl -X POST "$BASE_URL/email/send-to-lists" \
  -H "Content-Type: application/json" \
  -d '{
    "list_ids": [999],
    "subject": "Should Fail",
    "body": "This should not be sent"
  }'

# Test 5: Send with invalid email
echo -e "\n\n${GREEN}Test 5: Testing error handling - Invalid email${NC}"
curl -X POST "$BASE_URL/email/send" \
  -H "Content-Type: application/json" \
  -d '{
    "to": "not-an-email",
    "subject": "Should Fail",
    "body": "This should not be sent"
  }'

echo -e "\n\nTests completed!" 