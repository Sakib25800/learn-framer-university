# Plugin

The Framer plugin for Framer University that enhances the learning experience within Framer.

## Development

### Prerequisites

- Node.js 18+
- pnpm 8+
- Framer Desktop App

### Getting Started

1. Install dependencies:

```bash
pnpm install
```

2. Start the development server:

```bash
turbo dev --filter plugin
```

3. Open Framer Desktop App and load the plugin from `http://localhost:3004`

### Scripts

- `dev`: Start development server on port 3004 with host access
- `build`: Build the plugin for production using Vite
- `check-types`: Run TypeScript type checking without emitting files
- `lint`: Run ESLint to check for code quality issues
- `format`: Format code using Prettier
- `format:check`: Check if code is properly formatted
- `deploy`: Deploy the plugin to Cloudflare Workers
- `preview`: Preview the plugin deployment locally using Wrangler
- `clean`: Remove build artifacts (dist and .turbo directories)
- `pack`: Package the plugin for distribution using Framer Plugin Tools

### Deployment

- The plugin is deployed as a Cloudflare Worker for local use
- For production refer to [how to submit a plugin](https://www.framer.com/help/articles/how-to-submit-a-plugin-to-the-marketplace/)
