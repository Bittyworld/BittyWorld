# Build stage
FROM node:18-alpine AS builder

WORKDIR /app

# Copy package files
COPY package*.json ./
COPY client/package*.json ./client/
COPY server/package*.json ./server/

# Install dependencies
RUN npm install
RUN cd client && npm install
RUN cd server && npm install

# Copy source code
COPY . .

# Build client and server
RUN cd client && npm run build
RUN cd server && npm run build

# Production stage
FROM node:18-alpine

WORKDIR /app

# Copy built files
COPY --from=builder /app/client/.next ./client/.next
COPY --from=builder /app/server/dist ./server/dist
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/client/node_modules ./client/node_modules
COPY --from=builder /app/server/node_modules ./server/node_modules

# Copy configuration files
COPY package*.json ./
COPY client/package*.json ./client/
COPY server/package*.json ./server/

# Expose ports
EXPOSE 3000
EXPOSE 4000

# Start the application
CMD ["npm", "start"] 