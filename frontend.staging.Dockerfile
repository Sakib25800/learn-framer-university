ARG NODE_VERSION=23.6.0
FROM node:${NODE_VERSION}-slim AS base

ARG BACKEND_URL
ENV BACKEND_URL=${BACKEND_URL}
ENV NODE_ENV="staging"

LABEL fly_launch_runtime="Next.js"

WORKDIR /app

# Throw-away build stage to reduce size of final image
FROM base AS build

# Install packages needed to build node modules
RUN apt-get update -qq && \
    apt-get install --no-install-recommends -y build-essential node-gyp pkg-config python-is-python3

# Install node modules
COPY package-lock.json package.json ./
RUN npm ci

# Copy application code
COPY . .

# Build application with staging optimizations
RUN NEXT_PUBLIC_APP_ENV=staging npx next build

# Final stage for app image
FROM base

# Copy standalone build output
COPY --from=build /app/.next/standalone /app
COPY --from=build /app/.next/static /app/.next/static
COPY --from=build /app/public /app/public

# Create and copy entrypoint script
COPY docker-entrypoint.js /app/docker-entrypoint.js
RUN chmod +x /app/docker-entrypoint.js

# Start the server by default, this can be overwritten at runtime
EXPOSE 3000

# Use the entrypoint script
CMD ["node", "server.js"]
