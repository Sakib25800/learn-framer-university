import { defineConfig, devices } from "@playwright/test"
import dotenv from "dotenv"

// Load environment variables from .env file
dotenv.config()

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  testDir: "e2e",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: process.env.CI ? [["html"], ["junit", { outputFile: "playwright-results/junit.xml" }], ["list"]] : "html",
  use: {
    trace: "on-first-retry",
    baseURL: "http://localhost:3000",
  },
  projects: [
    {
      name: "setup",
      testMatch: /.*\.setup\.ts/,
    },
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
      dependencies: ["setup"],
    },
  ],
  webServer: [
    {
      command: "npm run dev",
      url: "http://localhost:3000",
      reuseExistingServer: !process.env.CI,
      stderr: "pipe",
    },
    {
      command: "cargo run",
      url: "http://localhost:8080",
      reuseExistingServer: !process.env.CI,
      stderr: "pipe",
    },
  ],
  timeout: 30000,
  expect: {
    timeout: 10000,
  },
  outputDir: "playwright-results/",
})
