import { test, expect } from "./test-auth"

/**
 * E2E test: full challenge flow.
 *
 * Prerequisites:
 * - Mock OIDC server running (npm run mock-oidc)
 * - Backend running with JWKS_URL=http://localhost:9001/.well-known/jwks.json
 * - Frontend dev server running with VITE_AUTH_AUTHORITY=http://localhost:9001
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

  // Step 4 — Answer the challenge question on its own page
  // The new item's auto-created event opens pre-expanded and links the active Book challenge
  // (backend auto-links new books), shown as a clickable card.
  const challengeCard = authenticatedPage.getByRole("link", { name: challengeName })
  await expect(challengeCard).toBeVisible()
  await expect(challengeCard).toContainText("0 / 1 vastattu")
  await challengeCard.click()

  await expect(authenticatedPage).toHaveURL(/library\/[a-f0-9-]+\/challenges\/[a-f0-9-]+/)
  await expect(authenticatedPage.getByText(questionText)).toBeVisible()

  // Click "Kyllä" (Yes) to answer.
  // Answers auto-save (no "Tallenna" button on this view) - wait for the save request to complete.
  const answerSaved = authenticatedPage.waitForResponse(
    (res) => res.url().includes("/answers/") && res.request().method() === "POST",
  )
  await authenticatedPage.getByText("Kyllä").click()
  await answerSaved
  await expect(authenticatedPage.getByText("1 / 1 vastattu")).toBeVisible()

  // Back on the entry page's Events tab, the card reflects the answer
  await authenticatedPage.getByTestId("back-button").click()
  await expect(challengeCard).toContainText("1 / 1 vastattu")

  // Editing the event: unlinking the challenge hides its card, relinking brings the answers back
  const editEvent = authenticatedPage.getByRole("button", { name: "Muokkaa tapahtumaa" })
  await editEvent.click()
  await authenticatedPage.getByRole("checkbox", { name: challengeName }).uncheck()
  await authenticatedPage.getByRole("button", { name: "Tallenna" }).click()
  await expect(challengeCard).toHaveCount(0)

  await editEvent.click()
  await authenticatedPage.getByRole("checkbox", { name: challengeName }).check()
  await authenticatedPage.getByRole("button", { name: "Tallenna" }).click()
  await expect(challengeCard).toContainText("1 / 1 vastattu")

  // A second event linking the same challenge shares the item's single answer set
  await authenticatedPage.getByRole("button", { name: "Lisää uusi" }).click()
  await authenticatedPage.getByRole("button", { name: "Tallenna" }).click()
  await expect(challengeCard).toHaveCount(2)
  for (const card of await challengeCard.all()) {
    await expect(card).toContainText("1 / 1 vastattu")
  }
  await challengeCard.last().click()
  await expect(authenticatedPage.getByRole("radio", { name: "Kyllä" })).toHaveAttribute(
    "aria-checked",
    "true",
  )

  // Going back leads answer page → item → library; the creation form isn't in the history
  await authenticatedPage.goBack()
  await expect(authenticatedPage).toHaveURL(/library\/[a-f0-9-]+$/)
  await authenticatedPage.getByTestId("back-button").click()
  await expect(authenticatedPage).toHaveURL(/\/library$/)

  // Step 5 — Verify the library listing page
  await authenticatedPage.goto("/library")

  await expect(authenticatedPage.locator(".card")).toHaveCount(1)
  await expect(authenticatedPage.getByText(bookName)).toBeVisible()
  await expect(authenticatedPage.getByText(authorName)).toBeVisible()

  // Step 6 — Verify the home page challenge card
  await authenticatedPage.goto("/home")

  // Challenges are shared between users, so scope checks to this test's own card
  const homeCard = authenticatedPage.getByRole("link", { name: challengeName })
  await expect(homeCard).toBeVisible()
  await expect(homeCard.getByText("1 / 1")).toBeVisible()
  await expect(homeCard.getByText("1 kirjaa")).toBeVisible()
  // Answered, but no solution chosen yet
  await expect(homeCard.getByText("0 ratkaistu")).toBeVisible()

  // Step 7 — Choose the book as the solution; the green solved count follows
  await homeCard.click()
  const solutionSaved = authenticatedPage.waitForResponse(
    (res) => res.url().includes("/solution/") && res.request().method() === "POST",
  )
  await authenticatedPage.locator("select").selectOption({ label: `✓✓ ${bookName} · 0/1` })
  await solutionSaved

  await authenticatedPage.goto("/home")
  await expect(homeCard.getByText("1 ratkaistu")).toBeVisible()

  // Step 8 — Deleting the solution item drops both counts
  await authenticatedPage.goto("/library")
  await authenticatedPage.getByRole("link", { name: authorName }).click()
  await authenticatedPage.getByRole("button", { name: "Poista" }).click()
  await expect(authenticatedPage).toHaveURL(/\/home$/)
  await expect(homeCard.getByText("0 / 1")).toBeVisible()
  await expect(homeCard.getByText("0 ratkaistu")).toBeVisible()
})
