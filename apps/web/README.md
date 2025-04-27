# Web

The learning platform for Framer University.

## Development

### Prerequisites

- Node.js 18+
- pnpm 8+

### Getting Started

1. Install dependencies:

```bash
pnpm install
```

2. Start the development server:

```bash
turbo dev --filter web
```

3. Open `http://localhost:3000` in your browser

### Scripts

- `dev`: Start development server on port 3000
- `build`: Build the application for production
- `start`: Start the production server
- `lint`: Run ESLint to check for code quality issues
- `check-types`: Run TypeScript type checking without emitting files
- `e2e`: Run Playwright end-to-end tests
- `e2e:ui`: Run Playwright end-to-end tests with UI mode
- `format`: Format code using Prettier
- `format:check`: Check if code is properly formatted
- `preview`: Build and preview the application using OpenNext for Cloudflare
- `deploy`: Build and deploy the application to Cloudflare
- `cf-typegen`: Generate Cloudflare environment types
- `analyze`: Build the application with bundle analysis enabled
- `clean`: Remove build artifacts (.next and .turbo directories)

## Deployment

- Deployed as Cloudflare Worker with OpenNext
- Cloudflare R2 for storage of incremental cache
- Cloudflare D1 for queue and tag cache for On-demand revalidations via revalidateTag and revalidatePath.
