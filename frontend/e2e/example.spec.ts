import { test, expect } from "./test-auth"

/**
 * E2E test using the real mock OIDC login flow.
 *
 * Prerequisites:
 * - Mock OIDC server running (npm run mock-oidc)
 * - Backend running with JWKS_URL=http://localhost:9000/.well-known/jwks.json
 * - Frontend dev server running with VITE_AUTH_AUTHORITY=http://localhost:9000
 */

test("User can log in and add a book to an empty library", async ({ authenticatedPage }) => {
  // Start at the landing page
  await authenticatedPage.goto("/")

  // Click login — triggers OIDC flow through mock server
  await authenticatedPage.getByRole("button", { name: "Kirjaudu sisään" }).click()

  // After OIDC callback completes, we should land on /home
  await authenticatedPage.waitForURL("/home")

  // Navigate to library
  await authenticatedPage.goto("/library")

  // Verify library is empty for this new user
  const newItemButton = authenticatedPage.getByRole("button", { name: "Lisää uusi" })
  await expect(newItemButton).toBeVisible()
  await expect(authenticatedPage.locator(".card")).toHaveCount(0)

  // Add a new book
  await newItemButton.click()

  const bookName = `Test Book ${Date.now()}`
  await authenticatedPage.getByLabel("Nimi").fill(bookName)
  await authenticatedPage.getByLabel("Kirjailija").fill("Test Author")

  const saveButton = authenticatedPage.getByRole("button", { name: "Tallenna" })
  await expect(saveButton).toBeEnabled()
  await saveButton.click()

  await expect(authenticatedPage).toHaveURL(/library\/[a-f0-9-]+/)
  await expect(authenticatedPage.getByText(bookName)).toBeVisible()
})
