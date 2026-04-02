#!/bin/bash
# Build script for musl-linux-mipsle target

set -e

echo "=== Building Sublink Worker for musl-linux-mipsle ==="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored messages
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if mipsel target is installed
if ! rustup target list --installed | grep -q "mipsel-unknown-linux-musl"; then
    print_info "Installing mipsel-unknown-linux-musl target..."
    rustup target add mipsel-unknown-linux-musl
fi

# Check if cross compiler is available
if ! command -v mipsel-linux-gnu-gcc &> /dev/null; then
    print_error "mipsel-linux-gnu-gcc not found."
    print_error "Please install the cross compiler:"
    echo ""
    print_info "On Ubuntu/Debian:"
    echo "  sudo apt-get update && sudo apt-get install gcc-mipsel-linux-gnu"
    echo ""
    print_info "On Arch Linux:"
    echo "  sudo pacman -S mipsel-linux-gnu-gcc"
    echo ""
    print_info "On Fedora:"
    echo "  sudo dnf install gcc-mipsel-linux-gnu"
    echo ""
    print_info "Alternatively, use Docker:"
    echo "  docker build -t sublink-worker-rust ."
    exit 1
fi

print_info "Cross compiler found: $(mipsel-linux-gnu-gcc --version | head -1)"

# Set environment variables for cross compilation
export CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_MUSL_LINKER=mipsel-linux-gnu-gcc
export CC_mipsel_unknown_linux_musl=mipsel-linux-gnu-gcc
export RUSTFLAGS="-C target-feature=+crt-static"

# Build
print_info "Building release binary for mipsel-unknown-linux-musl..."
cargo build --release --target mipsel-unknown-linux-musl --features static

# Show binary info
BINARY_PATH="target/mipsel-unknown-linux-musl/release/sublink-worker"
echo ""
print_info "=== Build Complete ==="
print_info "Binary: $BINARY_PATH"
print_info "Size: $(du -h $BINARY_PATH | cut -f1)"
echo ""

# Verify it's a mipsel binary
if command -v file &> /dev/null; then
    print_info "Binary info:"
    file $BINARY_PATH
    echo ""
fi

print_info "To run on a mipsel device:"
echo "  scp $BINARY_PATH root@your-device:/usr/local/bin/sublink-worker"
echo "  ssh root@your-device 'sublink-worker'"
