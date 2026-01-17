#!/bin/bash

# Test runner script for Re-Vibe
# This script sets up the test environment, runs all tests, and cleans up

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🧪 Re-Vibe Test Runner${NC}"
echo "================================"

# Check if .env.test exists
if [ ! -f .env.test ]; then
    echo -e "${YELLOW}⚠️  .env.test not found, creating from example...${NC}"
    cp env.test.example .env.test
    echo -e "${GREEN}✓ Created .env.test${NC}"
fi

# Start test database
echo -e "${YELLOW}🐘 Starting test database...${NC}"
docker-compose -f docker-compose.test.yml up -d

# Wait for database to be ready
echo -e "${YELLOW}⏳ Waiting for database to be ready...${NC}"
sleep 5

# Check if database is ready with retries
MAX_RETRIES=30
RETRY_COUNT=0
until docker exec re-vibe-test-db-1 pg_isready -U postgres > /dev/null 2>&1; do
    RETRY_COUNT=$((RETRY_COUNT+1))
    if [ $RETRY_COUNT -ge $MAX_RETRIES ]; then
        echo -e "${RED}✗ Database failed to become ready${NC}"
        docker-compose -f docker-compose.test.yml down -v
        exit 1
    fi
    echo -e "${YELLOW}   Waiting for PostgreSQL (attempt $RETRY_COUNT/$MAX_RETRIES)...${NC}"
    sleep 1
done
echo -e "${GREEN}✓ Database is ready${NC}"

# Additional wait to ensure database is fully initialized
sleep 2

# Run migrations
echo -e "${YELLOW}🔄 Running database migrations...${NC}"
DATABASE_URL=postgres://postgres:testpassword@localhost:5434/re_vibe_test diesel migration run
echo -e "${GREEN}✓ Migrations complete${NC}"

# Build tests first (leverages incremental compilation)
echo ""
echo -e "${YELLOW}🔨 Building tests...${NC}"
cargo build --tests --workspace

# Run tests
echo ""
echo -e "${YELLOW}🧪 Running tests...${NC}"
echo "================================"

# Run tests and capture exit code
set +e  # Don't exit on test failure
cargo test --workspace --no-fail-fast -- --test-threads=1
TEST_EXIT_CODE=$?
set -e

echo ""
echo "================================"

# Cleanup
echo -e "${YELLOW}🧹 Cleaning up...${NC}"
docker-compose -f docker-compose.test.yml down -v
echo -e "${GREEN}✓ Cleanup complete${NC}"

echo ""
if [ $TEST_EXIT_CODE -eq 0 ]; then
    echo -e "${GREEN}✅ All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed${NC}"
    exit $TEST_EXIT_CODE
fi
j