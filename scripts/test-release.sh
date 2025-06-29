#!/bin/bash
set -euo pipefail

# Test release script for local development
# This script tests the release process locally before pushing tags

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🔧 Testing release build process..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
    print_error "Cargo.toml not found. Please run this script from the project root or scripts directory."
    exit 1
fi

cd "$PROJECT_ROOT"

# Test basic compilation
echo "📦 Testing basic build..."
if cargo build --release; then
    print_status "Release build successful"
else
    print_error "Release build failed"
    exit 1
fi

# Test different targets (if cross is installed)
if command -v cross &> /dev/null; then
    echo "🎯 Testing cross-compilation targets..."
    
    TARGETS=(
        "x86_64-unknown-linux-gnu"
        "x86_64-unknown-linux-musl"
        "x86_64-pc-windows-gnu"
    )
    
    for target in "${TARGETS[@]}"; do
        echo "Building for $target..."
        if cross build --release --target "$target"; then
            print_status "Build successful for $target"
        else
            print_warning "Build failed for $target (this might be expected)"
        fi
    done
else
    print_warning "Cross not installed. Install with: cargo install cross"
    print_warning "Skipping cross-compilation tests"
fi

# Test that the binary works
echo "🧪 Testing binary functionality..."
BINARY_PATH="$PROJECT_ROOT/target/release/awscw"

if [ -f "$BINARY_PATH" ]; then
    # Test help command
    if "$BINARY_PATH" --help &> /dev/null; then
        print_status "Binary help command works"
    else
        print_error "Binary help command failed"
        exit 1
    fi
    
    # Get binary size
    BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
    print_status "Binary size: $BINARY_SIZE"
else
    print_error "Binary not found at $BINARY_PATH"
    exit 1
fi

# Test archive creation (similar to what GitHub Actions does)
echo "📁 Testing archive creation..."
cd "$PROJECT_ROOT/target/release"

# Test tar.gz creation
ARCHIVE_NAME="awscw-test.tar.gz"
if tar -czf "$ARCHIVE_NAME" awscw; then
    ARCHIVE_SIZE=$(du -h "$ARCHIVE_NAME" | cut -f1)
    print_status "Archive created: $ARCHIVE_NAME ($ARCHIVE_SIZE)"
    rm "$ARCHIVE_NAME"
else
    print_error "Failed to create archive"
    exit 1
fi

echo ""
echo "🎉 All tests passed! Ready for release."
echo ""
echo "To create a release:"
echo "1. Update version in Cargo.toml"
echo "2. Update CHANGELOG.md"
echo "3. Commit changes: git commit -am 'chore: bump version to vX.Y.Z'"
echo "4. Create and push tag: git tag vX.Y.Z && git push origin vX.Y.Z"
echo "5. GitHub Actions will automatically create the release"