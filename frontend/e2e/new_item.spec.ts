import { test, expect } from "@playwright/test"
test("User can save a book", async ({ page }) => {
  await page.goto("/")
  const newItemButton = page.getByRole("button", { name: "Lisää uusi"})
  await expect(newItemButton).toBeVisible()
  await newItemButton.click()

  // Adding a book

  await page.getByLabel("Nimi").fill("E2E Test Book")
  await page.getByLabel("Kirjailija").fill("Test Author")
  await page.getByLabel("Kääntäjä").fill("Test Translator")

  await page.getByRole("button", { name: "Tallenna" }).click()

  await expect(page).toHaveURL(/library\/\d+/)

  await expect(page.getByText("E2E Test Book")).toBeVisible()
  await expect(page.getByText("Test Author")).toBeVisible()
  await expect(page.getByText("Test Translator")).toBeVisible()

  await page.getByTestId("back-button").click()

  // Verify the new item is in the listing
  const newItemInList = page.getByText("E2E Test Book")
  await expect(newItemInList).toBeVisible()
})
