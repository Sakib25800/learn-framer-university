import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
    testDir: "e2e",
    fullyParallel: true,
    forbidOnly: !!process.env.CI,
    retries: process.env.CI ? 2 : 0,
    workers: process.env.CI ? 1 : undefined,
    reporter: process.env.CI
        ? [
            ["html"],
            ["junit", { outputFile: "playwright-results/junit.xml" }],
            ["list"],
        ]
        : "html",
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
    ],
    outputDir: "playwright-results/",
});
