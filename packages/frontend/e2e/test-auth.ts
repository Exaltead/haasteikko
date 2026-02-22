import { test as base, type Page } from "@playwright/test"

type TestFixtures = {
  authenticatedPage: Page
}

/**
 * Extended Playwright test with authentication via real OIDC flow.
 * Each test gets a unique browser context, so each triggers its own
 * mock OIDC login and gets a unique user automatically.
 *
 * Prerequisites:
 * - Mock OIDC server running (npm run mock-oidc)
 * - Backend running with JWKS_URL pointing to mock OIDC server
 * - Frontend running with VITE_AUTH_AUTHORITY pointing to mock OIDC server
 */
export const test = base.extend<TestFixtures>({
  authenticatedPage: async ({ page }, use) => {
    await use(page)
  },
})

export { expect } from "@playwright/test"
