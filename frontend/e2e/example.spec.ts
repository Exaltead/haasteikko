import { test, expect } from "./test-auth"

/**
 * Example test demonstrating the test auth system.
 * Each test gets a unique user ID, providing full isolation.
 *
 * Prerequisites:
 * - Mock OIDC server running (npm run mock-oidc)
 * - Backend running with JWKS_URL=http://localhost:9000/.well-known/jwks.json
 * - Frontend dev server running with VITE_AUTH_AUTHORITY=http://localhost:9000
 */

test("User can save a book with isolated test user", async ({ authenticatedPage }) => {
  // Navigate directly to library - auth is mocked
  await authenticatedPage.goto("/library")

  const newItemButton = authenticatedPage.getByRole("button", { name: "Lisää uusi" })
  await expect(newItemButton).toBeVisible()
  await newItemButton.click()

  // Adding a book with unique name for this test
  const bookName = `Test Book ${Date.now()}`
  await authenticatedPage.getByLabel("Nimi").fill(bookName)
  await authenticatedPage.getByLabel("Kirjailija").fill("Test Author")

  await authenticatedPage.getByRole("button", { name: "Tallenna" }).click()

  await expect(authenticatedPage).toHaveURL(/library\/\d+/)
  await expect(authenticatedPage.getByText(bookName)).toBeVisible()
})

test("Different test gets a different user", async ({ authenticatedPage, testUser }) => {
  // This test has a completely different user than the previous test
  // Any data created here won't affect other tests
  console.log(`Test running with user ID: ${testUser.userId}`)

  await authenticatedPage.goto("/library")

  // New user should have empty library
  await expect(authenticatedPage.getByRole("button", { name: "Lisää uusi" })).toBeVisible()
})
