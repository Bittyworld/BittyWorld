# BittyWorld Deployment Guide

This document describes how to deploy the BittyWorld platform, including the frontend, backend, and smart contracts.

## Prerequisites
- Node.js (v18+)
- Rust (latest stable)
- Solana CLI & Anchor
- Docker (optional, for containerized deployment)

## Steps

1. **Clone the repository**
   ```bash
   git clone https://github.com/Bittyworld/BittyWorld.git
   cd BittyWorld
   ```

2. **Install dependencies**
   ```bash
   cd client && npm install
   cd ../server && npm install
   cd ../contracts && anchor build
   ```

3. **Configure environment variables**
   - Copy `.env.example` to `.env` in each service and fill in the required values.

4. **Deploy smart contracts**
   ```bash
   cd contracts
   anchor deploy
   ```

5. **Start backend and frontend**
   ```bash
   cd server && npm run dev
   cd ../client && npm run dev
   ```

6. **(Optional) Use Docker Compose**
   ```bash
   docker-compose up --build
   ```

---

For more details, see the main README and each subproject's README. 