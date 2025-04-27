# Storybook

The component documentation and development environment for Framer University UI components.

## Development

### Prerequisites

- Node.js 18+
- pnpm 8+

### Getting Started

1. Install dependencies:

```bash
pnpm install
```

2. Start the Storybook development server:

```bash
turbo dev --filter storybook
```

3. Open `http://localhost:6006` in your browser

### Scripts

- `dev`: Start Storybook development server on port 6006
- `storybook`: Alias for dev command
- `build-storybook`: Build Storybook for production
- `preview`: Preview the built Storybook
- `lint`: Run ESLint to check for code quality issues
- `format`: Format code using Prettier
- `format:check`: Check if code is properly formatted

### Writing Stories

Stories are located in the `stories` directory.

Example story structure:

```tsx
import type { Meta, StoryObj } from "@storybook/react";
import { Button } from "./Button";

const meta = {
  title: "Components/Button",
  component: Button,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Button>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    variant: "primary",
    children: "Button",
  },
};
```

### Deployment

The Storybook instance is automatically deployed to Chromatic on every push to the main branch.
