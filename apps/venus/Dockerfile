# Use Node.js 22 Alpine image (latest available)
FROM node:22.12-alpine AS base

# Install pnpm
RUN npm install -g pnpm

# Set working directory
WORKDIR /app

# Copy package files
COPY package.json pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy source code
COPY . .

# Build the application
RUN pnpm run build

# Production stage
FROM node:22.12-alpine AS production

# Install pnpm
RUN npm install -g pnpm

WORKDIR /app

# Copy package files
COPY package.json pnpm-lock.yaml ./

# Install only production dependencies
RUN pnpm install --prod --frozen-lockfile

# Copy built application
COPY --from=base /app/dist ./dist

# Expose port
EXPOSE 3000

# Start the application
CMD ["pnpm", "preview", "--host", "0.0.0.0", "--port", "3000"]