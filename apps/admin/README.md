# Admin Dashboard

The admin dashboard for Framer University.

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
turbo dev --filter admin
```

3. Open `http://localhost:5173` in your browser

### Scripts

- `dev`: Start development server
- `build`: Build for production
- `preview`: Preview production build
- `lint`: Run ESLint
- `test`: Run tests

### Environment Variables

```env
VITE_API_URL=http://localhost:3001
VITE_LOOPS_API_KEY=your_loops_api_key
```

## Deployment

- Deployed as Cloudflare Worker
