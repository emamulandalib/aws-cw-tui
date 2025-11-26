#!/bin/bash
set -euo pipefail

# Setup script for installing git hooks
# Run this after cloning the repository to enable pre-commit checks

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

echo "Setting up git hooks for aws-cw-tui..."
echo ""

# Check if we're in a git repository
if [ ! -d "$PROJECT_ROOT/.git" ]; then
    print_error "Not a git repository. Please run this from within the project."
    exit 1
fi

# Check if hooks directory exists
if [ ! -d "$PROJECT_ROOT/hooks" ]; then
    print_error "Hooks directory not found at $PROJECT_ROOT/hooks"
    exit 1
fi

# Use git's core.hooksPath to point to our hooks directory
# This is the modern and recommended approach
cd "$PROJECT_ROOT"
git config core.hooksPath hooks
print_status "Configured git to use hooks directory"

# Ensure hooks are executable
chmod +x "$PROJECT_ROOT/hooks/"*
print_status "Made hooks executable"

echo ""
print_status "Git hooks installed successfully!"
echo ""
echo "The following hooks are now active:"
echo "  - pre-commit: Runs cargo fmt, clippy, test, build, and Markdown checks before each commit"
echo ""
echo "To skip hooks temporarily (not recommended), use: git commit --no-verify"
echo "To disable hooks, run: git config --unset core.hooksPath"
