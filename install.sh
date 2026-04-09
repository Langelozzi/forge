#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

REPO="langelozzi/forge"
BINARY_NAME="forge"

# Detect OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

# Map to release asset names
case "${OS}" in
  Darwin)
    ASSET_NAME="forge-macos"
    ;;
  Linux)
    ASSET_NAME="forge-linux"
    ;;
  *)
    echo -e "${RED}Error: Unsupported operating system: $OS${NC}"
    echo "Only macOS and Linux are supported."
    exit 1
    ;;
esac

echo -e "${YELLOW}Installing $BINARY_NAME for $OS ($ARCH)...${NC}"

# Get the latest release tag
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep -o '"tag_name": "[^"]*' | cut -d'"' -f4)

if [ -z "$LATEST_RELEASE" ]; then
  echo -e "${RED}Error: Could not find latest release${NC}"
  exit 1
fi

echo -e "${YELLOW}Latest release: $LATEST_RELEASE${NC}"

# Construct the download URL
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/$ASSET_NAME"

echo -e "${YELLOW}Downloading from: $DOWNLOAD_URL${NC}"

# Create temporary directory
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Download the binary (it will be named forge-<platform>)
DOWNLOADED_FILE="$TEMP_DIR/$ASSET_NAME"
if ! curl -fsSL -o "$DOWNLOADED_FILE" "$DOWNLOAD_URL"; then
  echo -e "${RED}Error: Failed to download $BINARY_NAME${NC}"
  exit 1
fi

# Make it executable
chmod +x "$DOWNLOADED_FILE"

# Rename to just 'forge'
mv "$DOWNLOADED_FILE" "$TEMP_DIR/$BINARY_NAME"

# Determine installation directory
INSTALL_DIR="$HOME/.local/bin"
if [ ! -d "$INSTALL_DIR" ]; then
  INSTALL_DIR="/usr/local/bin"
fi

# Check if we can write to installation directory
if [ ! -w "$INSTALL_DIR" ]; then
  echo -e "${YELLOW}$INSTALL_DIR is not writable. Using sudo...${NC}"
  sudo mv "$TEMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
else
  mv "$TEMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
fi

# Verify installation
if command -v "$BINARY_NAME" &> /dev/null; then
  VERSION=$($BINARY_NAME --version 2>/dev/null || echo "unknown")
  echo -e "${GREEN}✓ $BINARY_NAME installed successfully!${NC}"
  echo -e "${GREEN}✓ Location: $(command -v $BINARY_NAME)${NC}"
  echo -e "${GREEN}✓ Version: $VERSION${NC}"
else
  echo -e "${YELLOW}⚠ Warning: $BINARY_NAME was installed but is not in PATH${NC}"
  echo -e "${YELLOW}Make sure $INSTALL_DIR is in your \$PATH${NC}"
  echo "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc):"
  echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
fi
