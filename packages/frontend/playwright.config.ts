import { defineConfig, devices } from "@playwright/test"

import { existsSync } from "fs"
import path, { dirname } from "path"
import { fileURLToPath } from "url"
const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

// Load test environment for E2E tests. Already-set variables are not overridden, so
// .env.test takes precedence over .env.
for (const file of [".env.test", ".env"]) {
  const envPath = path.resolve(__dirname, file)
  if (existsSync(envPath)) process.loadEnvFile(envPath)
}

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  testDir: "./e2e",
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: "html",
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Base URL to use in actions like `await page.goto('')`. */
    baseURL: "http://localhost:5183/",

    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: "on-first-retry",
  },

  /* Configure projects for major browsers */
  projects: [
    // Isolated tests with unique users per test
    // Each test gets its own user via the mock OIDC server
    {
      name: "Desktop Firefox",
      testMatch: /.*\.spec\.ts/,
      testIgnore: /.*\.setup\.ts/,
      use: { ...devices["Desktop Firefox"] },
    },

    /* Test against mobile viewports. */
    // {
    //   name: 'Mobile Chrome',
    //   use: { ...devices['Pixel 5'] },
    // },
    // {
    //   name: 'Mobile Safari',
    //   use: { ...devices['iPhone 12'] },
    // },

    /* Test against branded browsers. */
    // {
    //   name: 'Microsoft Edge',
    //   use: { ...devices['Desktop Edge'], channel: 'msedge' },
    // },
    // {
    //   name: 'Google Chrome',
    //   use: { ...devices['Desktop Chrome'], channel: 'chrome' },
    // },
  ],

  /* Run servers before starting the tests */
  webServer: [
    {
      command: "OIDC_PORT=9001 npm run mock-oidc",
      url: "http://localhost:9001/health",
      reuseExistingServer: !process.env.CI,
    },
    {
      command:
        "cd ../../packages/backend && rm -f test-e2e.sqlite && PORT=3001 JWKS_URL=http://localhost:9001/.well-known/jwks.json REQUIRED_AUDIENCE=https://haasteikko.eu/api MIGRATIONS_PATH=migrations DATABASE_PATH=test-e2e.sqlite cargo run",
      url: "http://localhost:3001/api/ping",
      reuseExistingServer: !process.env.CI,
      timeout: 120_000,
    },
    {
      command: "npm run dev -- --mode test --port 5183 --strictPort",
      url: "http://localhost:5183/",
      reuseExistingServer: !process.env.CI,
    },
  ],
})
