import { test, expect } from "./test-auth"

/**
 * E2E test: full challenge flow.
 *
 * Prerequisites:
 * - Mock OIDC server running (npm run mock-oidc)
 * - Backend running with JWKS_URL=http://localhost:9000/.well-known/jwks.json
 * - Frontend dev server running with VITE_AUTH_AUTHORITY=http://localhost:9000
 */

test("User can create a challenge, add a book, answer a question, and verify results", async ({
  authenticatedPage,
}) => {
  const challengeName = `Test Challenge ${Date.now()}`
  const questionText = "Onko kirja romaani?"
  const bookName = `Test Book ${Date.now()}`
  const authorName = "Test Author"

  // Step 1 — Login
  await authenticatedPage.goto("/")
  await authenticatedPage.getByRole("button", { name: "Kirjaudu sisään" }).click()
  await authenticatedPage.waitForURL("/home")

  // Step 2 — Create a challenge with a Boolean question
  await authenticatedPage.goto("/manageChallenges")

  await authenticatedPage.getByRole("button", { name: "Luo uusi haaste" }).click()

  await authenticatedPage.getByLabel("Haasteen nimi").fill(challengeName)
  // targetMedia defaults to "Kirja" (Book), no change needed

  // Click the add-question button (the button inside the <ul> with the plus icon)
  await authenticatedPage.locator("ul > button").click()

  await authenticatedPage.getByLabel("Kysymys").fill(questionText)
  // Question type defaults to "Kyllä/Ei" (Boolean), no change needed

  await authenticatedPage.getByRole("button", { name: "Tallenna" }).click()

  // After saving, the challenge list re-appears with the new challenge visible
  await expect(authenticatedPage.getByText(challengeName)).toBeVisible()

  // Step 3 — Add a book via the library
  await authenticatedPage.goto("/library")

  await authenticatedPage.getByRole("button", { name: "Lisää uusi" }).click()

  await authenticatedPage.getByLabel("Nimi").fill(bookName)
  await authenticatedPage.getByLabel("Kirjailija").fill(authorName)

  await authenticatedPage.getByRole("button", { name: "Tallenna" }).click()

  // After saving, we're redirected to the entry view /library/:id
  await expect(authenticatedPage).toHaveURL(/library\/[a-f0-9-]+/)

  // Step 4 — Answer the challenge question on the entry page
  // The challenge tab should be visible (backend auto-links new books to active Book challenges)
  // Use getByRole("tab") to target the HeadlessUI Tab specifically (challenge name also appears as <h1>)
  const challengeTab = authenticatedPage.getByRole("tab", { name: challengeName })
  await expect(challengeTab).toBeVisible()
  await challengeTab.click()

  // The EntryChallenge component renders a RadioGroup for the Boolean question
  await expect(authenticatedPage.getByText(questionText)).toBeVisible()

  // Click "Kyllä" (Yes) to answer
  await authenticatedPage.getByText("Kyllä").click()

  // Save the answers
  await authenticatedPage.getByRole("button", { name: "Tallenna" }).click()

  // Step 5 — Verify the library listing page
  await authenticatedPage.goto("/library")

  await expect(authenticatedPage.locator(".card")).toHaveCount(1)
  await expect(authenticatedPage.getByText(bookName)).toBeVisible()
  await expect(authenticatedPage.getByText(authorName)).toBeVisible()

  // Step 6 — Verify the home page challenge card
  await authenticatedPage.goto("/home")

  await expect(authenticatedPage.getByText(challengeName)).toBeVisible()
  await expect(authenticatedPage.getByText("1 / 1")).toBeVisible()
  await expect(authenticatedPage.getByText("1 kirjaa")).toBeVisible()
})
