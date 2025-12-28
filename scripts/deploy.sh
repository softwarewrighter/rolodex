#!/bin/bash
set -e

# Configuration
DEPLOY_TARGET="${DEPLOY_TARGET:-wrighter.space}"
DEPLOY_PATH="${DEPLOY_PATH:-/var/www/wrighter.space/staging/rolodex}"
DEPLOY_USER="${DEPLOY_USER:-deploy}"

# Change to project root
cd "$(dirname "$0")/.."

echo "Building for production..."
trunk build --release --public-url "/staging/rolodex/"

echo ""
echo "Build complete! Files in ./dist/:"
ls -la dist/

echo ""
echo "To deploy to $DEPLOY_TARGET:"
echo ""
echo "  Option 1: Using rsync (recommended)"
echo "  rsync -avz --delete dist/ $DEPLOY_USER@$DEPLOY_TARGET:$DEPLOY_PATH/"
echo ""
echo "  Option 2: Using scp"
echo "  scp -r dist/* $DEPLOY_USER@$DEPLOY_TARGET:$DEPLOY_PATH/"
echo ""
echo "  Option 3: Manual upload"
echo "  Upload the contents of ./dist/ to https://wrighter.space/staging/rolodex/"
echo ""

# If running in CI or with --auto flag, actually deploy
if [ "$1" = "--auto" ]; then
    echo "Auto-deploying..."
    rsync -avz --delete dist/ "$DEPLOY_USER@$DEPLOY_TARGET:$DEPLOY_PATH/"
    echo "Deploy complete!"
fi
