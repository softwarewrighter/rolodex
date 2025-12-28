#!/bin/bash
set -e

echo "Building Rolodex application..."

# Change to project root
cd "$(dirname "$0")/.."

# Build with trunk in release mode, output to docs/ for GitHub Pages
# --public-url sets the base path for assets (needed for GitHub Pages subdirectory)
trunk build --release --dist docs --public-url /rolodex/

# Add .nojekyll file for GitHub Pages (prevents Jekyll processing)
touch docs/.nojekyll

# Copy favicon
cp favicon.ico docs/

echo "Build complete! Output in ./docs/ (for GitHub Pages)"
ls -la docs/
