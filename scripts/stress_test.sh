#!/bin/bash

# Configuration
API_URL="http://localhost:8080"
API_KEY="OlH2V4j/OMfBnxfUvsrjoiD9xcI+/ihMv1go8/hf2HI="
NUM_REQUESTS=2  # Reduced for initial testing
CONCURRENT_USERS=2  # Reduced for initial testing

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Debug mode
set -x

echo -e "${YELLOW}Starting API stress test...${NC}"
echo -e "${YELLOW}API URL: $API_URL${NC}"
echo -e "${YELLOW}Concurrent users: $CONCURRENT_USERS${NC}"
echo -e "${YELLOW}Requests per user: $NUM_REQUESTS${NC}"
echo

# Function to create a list
create_list() {
    echo -e "${YELLOW}Creating list for user $1...${NC}"
    local response=$(curl -v -X POST "$API_URL/api/lists" \
        -H "Content-Type: application/json" \
        -H "X-API-Key: $API_KEY" \
        -d '{
            "name": "Test List '"$1"'",
            "type": "public",
            "optin": "single",
            "tags": ["test", "stress"],
            "description": "Test list created during stress test"
        }' 2>&1)
    
    echo "Raw response: $response"
    
    # Extract status code from curl verbose output
    local http_code=$(echo "$response" | grep "< HTTP" | awk '{print $3}')
    echo "HTTP Status Code: $http_code"
    
    # Extract response body
    local body=$(echo "$response" | grep -v "< " | grep -v "> " | tail -n 1)
    echo "Response body: $body"
    
    if [ "$http_code" = "201" ]; then
        echo -e "${GREEN}Created list $1${NC}"
        echo "$body" | jq -r '.id' || echo "Failed to parse JSON response"
    else
        echo -e "${RED}Failed to create list $1: $http_code${NC}"
        echo -e "${RED}Response: $response${NC}"
        return 1
    fi
}

# Function to get lists with pagination
get_lists() {
    echo -e "${YELLOW}Getting lists...${NC}"
    local response=$(curl -v -X GET "$API_URL/api/lists?page=1&per_page=10" \
        -H "X-API-Key: $API_KEY" 2>&1)
    
    echo "Raw response: $response"
    
    # Extract status code from curl verbose output
    local http_code=$(echo "$response" | grep "< HTTP" | awk '{print $3}')
    echo "HTTP Status Code: $http_code"
    
    if [ "$http_code" = "200" ]; then
        echo -e "${GREEN}Retrieved lists${NC}"
    else
        echo -e "${RED}Failed to get lists: $http_code${NC}"
        echo -e "${RED}Response: $response${NC}"
        return 1
    fi
}

# Function to update a list
update_list() {
    local id=$1
    echo -e "${YELLOW}Updating list $id...${NC}"
    local response=$(curl -v -X PUT "$API_URL/api/lists/$id" \
        -H "Content-Type: application/json" \
        -H "X-API-Key: $API_KEY" \
        -d '{
            "name": "Updated Test List '"$id"'",
            "description": "Updated during stress test"
        }' 2>&1)
    
    echo "Raw response: $response"
    
    # Extract status code from curl verbose output
    local http_code=$(echo "$response" | grep "< HTTP" | awk '{print $3}')
    echo "HTTP Status Code: $http_code"
    
    if [ "$http_code" = "200" ]; then
        echo -e "${GREEN}Updated list $id${NC}"
    else
        echo -e "${RED}Failed to update list $id: $http_code${NC}"
        echo -e "${RED}Response: $response${NC}"
        return 1
    fi
}

# Function to delete a list
delete_list() {
    local id=$1
    echo -e "${YELLOW}Deleting list $id...${NC}"
    local response=$(curl -v -X DELETE "$API_URL/api/lists/$id" \
        -H "X-API-Key: $API_KEY" 2>&1)
    
    echo "Raw response: $response"
    
    # Extract status code from curl verbose output
    local http_code=$(echo "$response" | grep "< HTTP" | awk '{print $3}')
    echo "HTTP Status Code: $http_code"
    
    if [ "$http_code" = "200" ]; then
        echo -e "${GREEN}Deleted list $id${NC}"
    else
        echo -e "${RED}Failed to delete list $id: $http_code${NC}"
        echo -e "${RED}Response: $response${NC}"
        return 1
    fi
}

# Function to run a complete test cycle
run_test_cycle() {
    local user_id=$1
    echo -e "${YELLOW}Starting test cycle for user $user_id${NC}"
    local start_time=$(date +%s%N)
    
    # Create a list and get its ID
    local list_id=$(create_list "$user_id")
    echo "List ID: $list_id"
    
    if [ $? -eq 0 ] && [ ! -z "$list_id" ]; then
        echo -e "${GREEN}Successfully created list with ID: $list_id${NC}"
        
        # Get lists
        get_lists
        
        # Update the list
        update_list "$list_id"
        
        # Delete the list
        delete_list "$list_id"
    else
        echo -e "${RED}Failed to create list for user $user_id${NC}"
    fi
    
    local end_time=$(date +%s%N)
    local duration=$(( ($end_time - $start_time) / 1000000 ))
    echo -e "${GREEN}Test cycle $user_id completed in ${duration}ms${NC}"
}

# Run concurrent tests
echo -e "${YELLOW}Starting concurrent tests...${NC}"
for i in $(seq 1 $CONCURRENT_USERS); do
    echo -e "${YELLOW}Starting user $i${NC}"
    for j in $(seq 1 $NUM_REQUESTS); do
        echo -e "${YELLOW}Starting request $j for user $i${NC}"
        run_test_cycle "$i-$j"  # Removed & to run sequentially for debugging
    done
done

echo
echo -e "${GREEN}Stress test completed!${NC}" 