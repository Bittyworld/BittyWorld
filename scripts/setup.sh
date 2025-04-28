#!/bin/bash
# BittyWorld project setup script

set -e

echo "Installing client dependencies..."
cd ../client && npm install

echo "Installing server dependencies..."
cd ../server && npm install

echo "Building smart contracts..."
cd ../contracts && anchor build

echo "Setup complete!" 