import { defineConfig, devices } from "@playwright/test"

/**
 * Read environment variables from file.
 * https://github.com/motdotla/dotenv
 */
import dotenv from "dotenv"
import path, { dirname } from "path"
import { fileURLToPath } from "url"
const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

// Load test environment for E2E tests
dotenv.config({ path: [path.resolve(__dirname, ".env.test"), path.resolve(__dirname, ".env")] })

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
    baseURL: "http://localhost:5173/",

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
      command: "npm run mock-oidc",
      url: "http://localhost:9000/health",
      reuseExistingServer: !process.env.CI,
    },
    {
      command:
        "cd ../backend && rm -f test-e2e.sqlite && JWKS_URL=http://localhost:9000/.well-known/jwks.json REQUIRED_AUDIENCE=https://haasteikko.eu/api MIGRATIONS_PATH=migrations DATABASE_PATH=test-e2e.sqlite cargo run",
      url: "http://localhost:3000/api/ping",
      reuseExistingServer: !process.env.CI,
      timeout: 120_000,
    },
    {
      command: "npm run dev -- --mode test",
      url: "http://localhost:5173/",
      reuseExistingServer: !process.env.CI,
    },
  ],
})
