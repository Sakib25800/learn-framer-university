import reactRefresh from "eslint-plugin-react-refresh";
import { config as reactConfig } from "./react.js";

/** @type {import("eslint").Linter.Config} */
export const config = [
  ...reactConfig,
  {
    files: ["**/*.{ts,tsx}"],
    plugins: {
      "react-refresh": reactRefresh,
    },
    rules: {
      "react-refresh/only-export-components": [
        "warn",
        { allowConstantExport: true },
      ],
    },
  },
  {
    ignores: ["**/storybook-static/**"],
  },
];
