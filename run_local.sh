#!/bin/bash

# Local development runner script for Re-Vibe
# This script starts the database and runs the application locally

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Re-Vibe Local Development Runner${NC}"
echo "===================================="

# Check if .env exists
if [ ! -f .env ]; then
    echo -e "${RED}✗ .env file not found${NC}"
    echo -e "${YELLOW}Please create a .env file with required environment variables${NC}"
    echo "See LOCAL_DEVELOPMENT.md for details"
    exit 1
fi

# Source .env file
export $(cat .env | grep -v '^#' | xargs)

# Check if diesel CLI is installed
if ! command -v diesel &> /dev/null; then
    echo -e "${YELLOW}⚠️  Diesel CLI not found. Installing...${NC}"
    cargo install diesel_cli --no-default-features --features postgres
    echo -e "${GREEN}✓ Diesel CLI installed${NC}"
fi

# Start database
echo -e "${YELLOW}🐘 Starting database...${NC}"
docker-compose up db -d

# Wait for database to be ready
echo -e "${YELLOW}⏳ Waiting for database to be ready...${NC}"
sleep 3

# Check if database is ready with retries
MAX_RETRIES=30
RETRY_COUNT=0
until docker exec re-vibe-db-1 pg_isready -U postgres > /dev/null 2>&1; do
    RETRY_COUNT=$((RETRY_COUNT+1))
    if [ $RETRY_COUNT -ge $MAX_RETRIES ]; then
        echo -e "${RED}✗ Database failed to become ready${NC}"
        docker-compose down
        exit 1
    fi
    echo -e "${YELLOW}   Waiting for PostgreSQL (attempt $RETRY_COUNT/$MAX_RETRIES)...${NC}"
    sleep 1
done
echo -e "${GREEN}✓ Database is ready${NC}"

# Run migrations
echo -e "${YELLOW}🔄 Running database migrations...${NC}"
diesel migration run
echo -e "${GREEN}✓ Migrations complete${NC}"

echo ""
echo -e "${GREEN}✅ Setup complete!${NC}"
echo ""
echo -e "${BLUE}Starting application...${NC}"
echo "===================================="
echo ""

# Run the application
cargo run --bin main --release
