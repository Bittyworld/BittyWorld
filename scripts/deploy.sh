#!/bin/bash
# BittyWorld deployment script (placeholder)

echo "Deploying BittyWorld smart contracts..."
cd ../contracts && anchor deploy

echo "Starting backend..."
cd ../server && npm run start &

echo "Starting frontend..."
cd ../client && npm run start &

echo "Deployment complete!" 