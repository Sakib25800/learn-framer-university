import type { Preview } from "@storybook/react";
import "@framer-university/ui/styles.css";

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/,
      },
    },
    backgrounds: {
      values: [
        { name: "Framer", value: "#1D1D1D" },
        { name: "Dark", value: "#000000" },
      ],
      default: "Framer",
    },
  },
};

export default preview;
