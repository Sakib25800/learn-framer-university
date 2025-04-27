# Turborepo Remote Cache Setup with Cloudflare Workers

## GitHub Repository Settings

You need to configure the following in your GitHub repository settings (Settings → Secrets and variables → Actions):

### Repository Variables (vars)

1. **TURBO_API**: `https://turborepo-remote-cache.frameruniversity.workers.dev`
2. **TURBO_TEAM**: `team_framer-university`

### Repository Secrets (secrets)

1. **TURBO_TOKEN**: `RgmUW3xyFqDdB4ZvWl6fQEldXew1FtXAN8HmJRscGfs=`
2. **TURBO_REMOTE_CACHE_SIGNATURE_KEY**: `RgmUW3xyFqDdB4ZvWl6fQEldXew1FtXAN8HmJRscGfs=`

## Local Testing

To test remote caching locally:

```bash
# Export environment variables
export TURBO_API=https://turborepo-remote-cache.frameruniversity.workers.dev
export TURBO_TEAM=team_framer-university
export TURBO_TOKEN=RgmUW3xyFqDdB4ZvWl6fQEldXew1FtXAN8HmJRscGfs=
export TURBO_REMOTE_CACHE_SIGNATURE_KEY=RgmUW3xyFqDdB4ZvWl6fQEldXew1FtXAN8HmJRscGfs=

# Force rebuild to populate cache
turbo run build --force

# Run again to verify cache hits
turbo run build
```

## Verification

When remote caching is working correctly, you should see:

- "Remote caching enabled" message
- "cache hit, replaying logs" for cached tasks
- Significantly faster build times on subsequent runs

## Troubleshooting

1. If you see "remote caching disabled", check:

   - All environment variables are set correctly
   - The Cloudflare Worker is accessible
   - The token and team name match your Cloudflare setup

2. If cache misses occur when they shouldn't:
   - Ensure TURBO_REMOTE_CACHE_SIGNATURE_KEY is the same across all environments
   - Check that file inputs haven't changed
   - Verify the same Node.js version is used locally and in CI
