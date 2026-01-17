#!/bin/bash

# Script to test CI workflow locally using act
# This simulates GitHub Actions on your local machine

set -e

# Colors for output
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${BLUE}🧪 Testing CI Locally with act${NC}"
echo "===================================="

# Check if act is installed
if ! command -v act &> /dev/null; then
    echo -e "${YELLOW}⚠️  act is not installed${NC}"
    echo ""
    echo "Install act with:"
    echo "  macOS:  brew install act"
    echo "  Linux:  curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash"
    echo ""
    exit 1
fi

echo ""
echo "Available options:"
echo "  1) Run CI tests workflow"
echo "  2) Run production build workflow"
echo "  3) List all workflows"
echo "  4) Dry run (show what would happen)"
echo ""
read -p "Select option (1-4): " option

case $option in
    1)
        echo -e "${GREEN}Running CI tests workflow...${NC}"
        act push -W .github/workflows/ci.yml
        ;;
    2)
        echo -e "${GREEN}Running production workflow...${NC}"
        echo -e "${YELLOW}Note: This requires Docker Hub credentials${NC}"
        act push -W .github/workflows/prod.yml --secret-file .secrets
        ;;
    3)
        echo -e "${GREEN}Available workflows:${NC}"
        act -l
        ;;
    4)
        echo -e "${GREEN}Dry run of CI workflow:${NC}"
        act push -W .github/workflows/ci.yml -n
        ;;
    *)
        echo "Invalid option"
        exit 1
        ;;
esac
