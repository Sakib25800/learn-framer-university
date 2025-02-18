import type { Preview } from "@storybook/react"

import "../app/globals.css"

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/,
      },
    },
    actions: { argTypesRegex: "^on[A-Z].*" },
    backgrounds: {
      values: [
        { name: "Framer", value: "#1D1D1D" },
        { name: "Dark", value: "#000000" },
      ],
      default: "Framer",
    },
  },
}

export default preview
