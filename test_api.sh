#!/bin/bash

# GResources API Test Script
# This script tests all the CRUD operations of the GResources API

# Configuration
BASE_URL="http://127.0.0.1:8080"
TEST_FOLDER="/test-folder"

# Generate a random resource name
RANDOM_ID=$((RANDOM % 9000 + 1000))
RESOURCE_NAME="test-resource-$RANDOM_ID"
RESOURCE_PATH="$TEST_FOLDER/$RESOURCE_NAME"
FULL_URL="$BASE_URL$RESOURCE_PATH"
FOLDER_URL="$BASE_URL$TEST_FOLDER"

echo "=== GResources API Test Script ==="
echo "Testing resource: $RESOURCE_PATH"
echo "Server URL: $BASE_URL"
echo ""

# Test content
ORIGINAL_CONTENT="This is the original content for resource $RESOURCE_NAME"
UPDATED_CONTENT="This is the UPDATED content for resource $RESOURCE_NAME"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
GRAY='\033[0;37m'
NC='\033[0m' # No Color

# Function to check HTTP status and display result
test_http_response() {
    local test_name="$1"
    local expected_status="$2"
    local actual_status="$3"
    local response_body="$4"
    
    if [ "$actual_status" -eq "$expected_status" ]; then
        echo -e "${GREEN}✅ $test_name - Expected: $expected_status, Got: $actual_status${NC}"
        if [ -n "$response_body" ]; then
            echo -e "${GRAY}   Response: $response_body${NC}"
        fi
        return 0
    else
        echo -e "${RED}❌ $test_name - Expected: $expected_status, Got: $actual_status${NC}"
        if [ -n "$response_body" ]; then
            echo -e "${GRAY}   Response: $response_body${NC}"
        fi
        return 1
    fi
}

# Function to make curl request and get status code only
curl_status_only() {
    local method="$1"
    local url="$2"
    local body="$3"
    
    if [ -n "$body" ]; then
        curl -X "$method" "$url" -d "$body" -w "%{http_code}" -s -o /dev/null
    else
        curl -X "$method" "$url" -w "%{http_code}" -s -o /dev/null
    fi
}

# Function to make curl request and get both status and body
curl_with_body() {
    local method="$1"
    local url="$2"
    local body="$3"
    local temp_file=$(mktemp)
    
    if [ -n "$body" ]; then
        local status=$(curl -X "$method" "$url" -d "$body" -w "%{http_code}" -s -o "$temp_file")
    else
        local status=$(curl -X "$method" "$url" -w "%{http_code}" -s -o "$temp_file")
    fi
    
    local response_body=$(cat "$temp_file")
    rm "$temp_file"
    
    echo "$status|$response_body"
}

echo -e "${CYAN}1. Testing GET non-existent resource (should be 404)...${NC}"
status=$(curl_status_only "GET" "$FULL_URL")
test_http_response "GET non-existent resource" 404 "$status"
echo ""

echo -e "${CYAN}2. Testing POST create resource (should be 201)...${NC}"
result=$(curl_with_body "POST" "$FULL_URL" "$ORIGINAL_CONTENT")
status=$(echo "$result" | cut -d'|' -f1)
body=$(echo "$result" | cut -d'|' -f2-)
test_http_response "POST create resource" 201 "$status" "$body"
echo ""

echo -e "${CYAN}3. Testing POST create same resource again (should be 409 Conflict)...${NC}"
result=$(curl_with_body "POST" "$FULL_URL" "$ORIGINAL_CONTENT")
status=$(echo "$result" | cut -d'|' -f1)
body=$(echo "$result" | cut -d'|' -f2-)
test_http_response "POST create duplicate resource" 409 "$status" "$body"
echo ""

echo -e "${CYAN}4. Testing GET existing resource (should be 200)...${NC}"
result=$(curl_with_body "GET" "$FULL_URL")
status=$(echo "$result" | cut -d'|' -f1)
body=$(echo "$result" | cut -d'|' -f2-)
if test_http_response "GET existing resource" 200 "$status" "$body"; then
    if [ "$body" = "$ORIGINAL_CONTENT" ]; then
        echo -e "${GREEN}✅ Content matches original${NC}"
    else
        echo -e "${RED}❌ Content doesn't match. Expected: '$ORIGINAL_CONTENT', Got: '$body'${NC}"
    fi
fi
echo ""

echo -e "${CYAN}5. Testing GET folder listing (should be 200 and contain resource)...${NC}"
result=$(curl_with_body "GET" "$FOLDER_URL")
status=$(echo "$result" | cut -d'|' -f1)
body=$(echo "$result" | cut -d'|' -f2-)
if test_http_response "GET folder listing" 200 "$status"; then
    if echo "$body" | grep -q "$RESOURCE_PATH"; then
        echo -e "${GREEN}✅ Resource found in folder listing${NC}"
    else
        echo -e "${RED}❌ Resource NOT found in folder listing${NC}"
        echo -e "${GRAY}   Folder contents: $body${NC}"
    fi
fi
echo ""

echo -e "${CYAN}6. Testing PATCH update resource (should be 204)...${NC}"
result=$(curl_with_body "PATCH" "$FULL_URL" "$UPDATED_CONTENT")
status=$(echo "$result" | cut -d'|' -f1)
body=$(echo "$result" | cut -d'|' -f2-)
test_http_response "PATCH update resource" 204 "$status" "$body"
echo ""

echo -e "${CYAN}7. Testing GET updated resource (should be 200 with new content)...${NC}"
result=$(curl_with_body "GET" "$FULL_URL")
status=$(echo "$result" | cut -d'|' -f1)
body=$(echo "$result" | cut -d'|' -f2-)
if test_http_response "GET updated resource" 200 "$status" "$body"; then
    if [ "$body" = "$UPDATED_CONTENT" ]; then
        echo -e "${GREEN}✅ Content matches updated content${NC}"
    else
        echo -e "${RED}❌ Content doesn't match updated. Expected: '$UPDATED_CONTENT', Got: '$body'${NC}"
    fi
fi
echo ""

echo -e "${CYAN}11. Testing DELETE resource (should be 200)...${NC}"
status=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$FULL_URL")
test_http_response "DELETE resource" 200 "$status" "(no body expected)"
echo ""

echo -e "${CYAN}9. Testing GET deleted resource (should be 404)...${NC}"
status=$(curl_status_only "GET" "$FULL_URL")
test_http_response "GET deleted resource" 404 "$status"
echo ""

echo -e "${CYAN}10. Testing GET folder listing after deletion (resource should not be listed)...${NC}"
result=$(curl_with_body "GET" "$FOLDER_URL")
status=$(echo "$result" | cut -d'|' -f1)
body=$(echo "$result" | cut -d'|' -f2-)
if test_http_response "GET folder listing after deletion" 200 "$status"; then
    if ! echo "$body" | grep -q "$RESOURCE_PATH"; then
        echo -e "${GREEN}✅ Resource correctly removed from folder listing${NC}"
    else
        echo -e "${RED}❌ Resource still found in folder listing!${NC}"
        echo -e "${GRAY}   Folder contents: $body${NC}"
    fi
fi
echo ""

echo -e "${GREEN}=== Test Complete ===${NC}"
echo -e "${YELLOW}All tests have been executed. Check the results above.${NC}"
