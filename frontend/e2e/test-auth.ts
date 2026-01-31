import { test as base, type Page } from "@playwright/test"

const MOCK_OIDC_URL = process.env.MOCK_OIDC_URL || "http://localhost:9000"
const API_URL = process.env.VITE_API_URL || "http://localhost:3000/api"

export interface TestUser {
  userId: string
  token: string
}

/**
 * Fetches a test token from the mock OIDC server
 */
export async function getTestToken(userId: string): Promise<string> {
  const response = await fetch(`${MOCK_OIDC_URL}/test/token`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ userId }),
  })

  if (!response.ok) {
    throw new Error(
      `Failed to get test token: ${response.status}. Is the mock OIDC server running?`,
    )
  }

  const data = await response.json()
  return data.token
}

/**
 * Sets up API request interception to inject the test token
 */
export async function setupApiAuth(page: Page, token: string): Promise<void> {
  await page.route(`${API_URL}/**`, async (route) => {
    const headers = {
      ...route.request().headers(),
      Authorization: `Bearer ${token}`,
    }
    await route.continue({ headers })
  })
}

/**
 * Mocks the OIDC authentication in the browser.
 * Stores a fake user session that the AuthService will recognize.
 */
export async function mockBrowserAuth(page: Page, userId: string, token: string): Promise<void> {
  // Create mock OIDC user object that oidc-client-ts expects
  const mockUser = {
    access_token: token,
    token_type: "Bearer",
    profile: {
      sub: userId,
      email: `${userId}@test.local`,
      email_verified: true,
    },
    expires_at: Math.floor(Date.now() / 1000) + 3600,
    expired: false,
  }

  // Store in sessionStorage where oidc-client-ts looks for it
  const storageKey = `oidc.user:${MOCK_OIDC_URL}:test-client`

  await page.addInitScript(
    ({ key, user }) => {
      sessionStorage.setItem(key, JSON.stringify(user))
    },
    { key: storageKey, user: mockUser },
  )
}

/**
 * Creates a unique user ID for test isolation
 */
export function generateTestUserId(testName: string): string {
  const timestamp = Date.now()
  const random = Math.random().toString(36).substring(2, 8)
  const sanitized = testName.replace(/[^a-zA-Z0-9]/g, "_").substring(0, 20)
  return `test_${sanitized}_${timestamp}_${random}`
}

// Extended test fixture with authenticated user
type TestFixtures = {
  testUser: TestUser
  authenticatedPage: Page
}

/**
 * Extended Playwright test with automatic test user authentication.
 * Each test gets a unique user, providing full isolation.
 *
 * Prerequisites:
 * - Mock OIDC server running (npm run mock-oidc)
 * - Backend running with JWKS_URL pointing to mock OIDC server
 * - Frontend running with VITE_AUTH_AUTHORITY pointing to mock OIDC server
 */
export const test = base.extend<TestFixtures>({
  testUser: async ({}, use, testInfo) => {
    const userId = generateTestUserId(testInfo.title)
    const token = await getTestToken(userId)
    await use({ userId, token })
  },

  authenticatedPage: async ({ page, testUser }, use) => {
    // Set up API request interception to add auth header
    await setupApiAuth(page, testUser.token)

    // Mock browser auth state
    await mockBrowserAuth(page, testUser.userId, testUser.token)

    await use(page)
  },
})

export { expect } from "@playwright/test"
