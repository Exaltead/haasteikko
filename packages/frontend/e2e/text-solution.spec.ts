import { test, expect } from "./test-auth"

/**
 * E2E test: a text question with the default cluster size (1) can be solved on the challenge
 * solution page, and the solved count on the home page follows.
 */
test("Text question with cluster size 1 can be given a solution", async ({ authenticatedPage }) => {
  const challengeName = `Text Challenge ${Date.now()}`
  const questionText = "Missä kirja tapahtuu?"
  const bookName = `Text Book ${Date.now()}`
  const authorName = "Text Author"

  await authenticatedPage.goto("/")
  await authenticatedPage.getByRole("button", { name: "Kirjaudu sisään" }).click()
  await authenticatedPage.waitForURL("/home")

  // Challenge with a single text question, cluster size left at its default of 1
  await authenticatedPage.goto("/manageChallenges")
  await authenticatedPage.getByRole("button", { name: "Luo uusi haaste" }).click()
  await authenticatedPage.getByLabel("Haasteen nimi").fill(challengeName)
  await authenticatedPage.locator("ul > button").click()
  await authenticatedPage.getByLabel("Kysymys", { exact: true }).fill(questionText)
  await authenticatedPage.getByLabel("Kysymyksen tyyppi").selectOption({ label: "Tekstisyöte" })
  await authenticatedPage.getByRole("button", { name: "Tallenna" }).click()
  await expect(authenticatedPage.getByText(challengeName)).toBeVisible()

  // Book auto-linked to the active challenge; answer the text question on its answer page
  await authenticatedPage.goto("/library")
  await authenticatedPage.getByRole("button", { name: "Lisää uusi" }).click()
  await authenticatedPage.getByLabel("Nimi").fill(bookName)
  await authenticatedPage.getByLabel("Kirjailija").fill(authorName)
  await authenticatedPage.getByRole("button", { name: "Tallenna" }).click()
  await expect(authenticatedPage).toHaveURL(/library\/[a-f0-9-]+/)

  await authenticatedPage.getByRole("link", { name: challengeName }).click()
  const answerSaved = authenticatedPage.waitForResponse(
    (res) => res.url().includes("/answers/") && res.request().method() === "POST",
  )
  await authenticatedPage.getByLabel(questionText).fill("Helsinki")
  await answerSaved

  // The solution page offers the book as the solution to the text question
  // Challenges are shared between users, so scope checks to this test's own card
  await authenticatedPage.goto("/home")
  const homeCard = authenticatedPage.getByRole("link", { name: challengeName })
  await expect(homeCard.getByText("1 / 1")).toBeVisible()
  await expect(homeCard.getByText("0 ratkaistu")).toBeVisible()
  await homeCard.click()

  const picker = authenticatedPage.getByRole("combobox")
  await expect(picker).toHaveCount(1)
  const solutionSaved = authenticatedPage.waitForResponse(
    (res) => res.url().includes("/solution/") && res.request().method() === "POST",
  )
  // Unused item that fits one question: unique, used 0 / possible 1
  await picker.selectOption({ label: `✓✓ ${bookName} · 0/1` })
  await solutionSaved
  // The picked option shows just the title, and the empty option turns into a clear action
  await expect(picker.locator("option:checked")).toHaveText(bookName)
  await expect(picker.locator("option").first()).toHaveText("— Tyhjennä —")

  // The choice survives a reload and counts as solved
  await authenticatedPage.reload()
  await expect(authenticatedPage.getByRole("combobox")).toHaveValue(/[a-f0-9-]{36}/)
  await authenticatedPage.goto("/home")
  await expect(homeCard.getByText("1 ratkaistu")).toBeVisible()
  await homeCard.click()

  // Clearing the single question from its dropdown removes the solution
  const cleared = authenticatedPage.waitForResponse(
    (res) => res.url().includes("/solution/") && res.request().method() === "POST",
  )
  await authenticatedPage.getByRole("combobox").selectOption({ label: "— Tyhjennä —" })
  await cleared
  await authenticatedPage.reload()
  await expect(authenticatedPage.getByRole("combobox")).toHaveValue("")
  await expect(authenticatedPage.locator("option:checked")).toHaveText("— Ei ratkaisua —")

  // Pick again, then reset the whole solution through the confirm dialog
  const repicked = authenticatedPage.waitForResponse(
    (res) => res.url().includes("/solution/") && res.request().method() === "POST",
  )
  await authenticatedPage.getByRole("combobox").selectOption({ label: `✓✓ ${bookName} · 0/1` })
  await repicked
  const reset = authenticatedPage.waitForResponse(
    (res) => res.url().includes("/solution/") && res.request().method() === "POST",
  )
  await authenticatedPage.getByRole("button", { name: "Tyhjennä ratkaisu" }).click()
  await authenticatedPage.getByRole("button", { name: "Tyhjennä", exact: true }).click()
  await reset
  await authenticatedPage.reload()
  await expect(authenticatedPage.getByRole("combobox")).toHaveValue("")
  await authenticatedPage.goto("/home")
  await expect(homeCard.getByText("0 ratkaistu")).toBeVisible()
})
