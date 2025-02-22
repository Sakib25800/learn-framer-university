ARG NODE_VERSION=23.6.0
FROM node:${NODE_VERSION}-slim AS base

ARG NEXT_PUBLIC_API_URL
ENV NEXT_PUBLIC_API_URL=${NEXT_PUBLIC_API_URL}

ARG NEXT_PUBLIC_APP_URL
ENV NEXT_PUBLIC_APP_URL=${NEXT_PUBLIC_APP_URL}

LABEL fly_launch_runtime="Next.js"

WORKDIR /app

FROM base AS build

RUN apt-get update -qq && \
    apt-get install --no-install-recommends -y build-essential node-gyp pkg-config python-is-python3

# Install pnpm
RUN npm install -g pnpm

# Install node modules
COPY pnpm-lock.yaml ./
COPY package.json ./
RUN pnpm install --frozen-lockfile

# Copy application code
COPY . .

# Build application
RUN NEXT_PUBLIC_APP_ENV=staging pnpm exec next build

FROM base

# Copy standalone build output
COPY --from=build /app/.next/standalone /app
COPY --from=build /app/.next/static /app/.next/static
COPY --from=build /app/public /app/public

# Create and copy entrypoint script
COPY docker-entrypoint.js /app/docker-entrypoint.js
RUN chmod +x /app/docker-entrypoint.js

EXPOSE 3000

# Use the entrypoint script
CMD ["node", "server.js"]
