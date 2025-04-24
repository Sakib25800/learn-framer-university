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
      command: "pnpm dev",
      url: "http://localhost:3000",
      reuseExistingServer: !process.env.CI,
      stderr: "pipe",
      env: {
        NEXT_PUBLIC_API_URL: "http://localhost:8080",
        NEXT_PUBLIC_APP_URL: "http://localhost:3000",
      },
    },
    {
      command: "./target/debug/server",
      url: "http://localhost:8080",
      reuseExistingServer: !process.env.CI,
      stderr: "pipe",
      timeout: 180000,
      env: {
        ALLOWED_ORIGINS: "http://localhost:3000",
        JWT_SECRET: "test_secret",
        JWT_ACCESS_TOKEN_EXPIRATION_HOURS: "1",
        JWT_REFRESH_TOKEN_EXPIRATION_DAYS: "7",
        EMAIL_VERIFICATION_EXPIRATION_HOURS: "24",
        CONNECTION_TIMEOUT_SECONDS: "1",
        POOL_SIZE: "5",
        APP_URL: "http://localhost:3000",
        DOMAIN_NAME: "localhost",
        DATABASE_URL: process.env.DATABASE_URL!,
        RUNNER_TEMP: process.env.RUNNER_TEMP!,
      },
    },
  ],
  timeout: 30000,
  expect: {
    timeout: 10000,
  },
  outputDir: "playwright-results/",
})
