import js from "@eslint/js";
import eslintConfigPrettier from "eslint-config-prettier";
import turboPlugin from "eslint-plugin-turbo";
import turboConfig from "eslint-config-turbo/flat";
import tseslint from "typescript-eslint";

/** @type {import("eslint").Linter.Config} */
export const config = [
  js.configs.recommended,
  eslintConfigPrettier,
  ...tseslint.configs.recommended,
  {
    plugins: {
      turbo: turboPlugin,
      turboConfig,
    },
  },
  {
    rules: {
      "@typescript-eslint/no-unused-expressions": ["error", {
        allowShortCircuit: true,
        allowTernary: true,
        allowTaggedTemplates: true
      }]
    }
  },
  {
    ignores: ["dist/**"],
  },
];
