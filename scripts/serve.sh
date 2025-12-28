#!/bin/bash
set -e

# Change to project root
cd "$(dirname "$0")/.."

echo "Starting development server..."
echo "App will be available at http://localhost:8080"

trunk serve --port 8080
