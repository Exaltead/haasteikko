import { test as setup, expect } from "@playwright/test"
import path from "path"
import { dirname } from "path"
import { fileURLToPath } from "url"

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)
const authFile = path.join(__dirname, "../playwright/.auth/user.json")

setup("authenticate", async ({ page }) => {
  const username = process.env.E2E_USERNAME!
  const password = process.env.E2E_PASSWORD!

  await page.goto("")

  const locator = page.getByText("Kirjaudu sisään")
  await locator.click()

  await page.getByRole("textbox", { name: "Email address" }).fill(username)
  await page.getByRole("textbox", { name: "Password" }).fill(password)
  await page.getByRole("button", { name: "Continue" }).click()

  // Expect a title "to contain" a substring.
  await expect(page).toHaveURL(/.*home/)

  await page.context().storageState({ path: authFile })
})
