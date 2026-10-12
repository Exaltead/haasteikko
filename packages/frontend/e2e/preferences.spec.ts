import { test, expect } from "./test-auth"

test("Library filter preferences persist across reloads", async ({ authenticatedPage }) => {
  await authenticatedPage.goto("/")
  await authenticatedPage.getByRole("button", { name: "Kirjaudu sisään" }).click()
  await authenticatedPage.waitForURL("/home")

  await authenticatedPage.goto("/library")
  await authenticatedPage.locator("main button").filter({ hasNotText: "Lisää uusi" }).click()

  // Change both filters; each change is saved to the backend
  const currentYear = String(new Date().getFullYear())
  const currentYearLabel = `Nykyinen(${currentYear})`
  const yearSelect = authenticatedPage.getByLabel("Vuosi")
  const checkboxes = authenticatedPage.getByRole("checkbox")
  await expect(checkboxes).toHaveCount(2)

  const saved = (body: string) =>
    authenticatedPage.waitForResponse(
      (resp) =>
        resp.url().endsWith("/api/preferences") &&
        resp.request().method() === "PUT" &&
        resp.request().postData()?.includes(body) === true,
    )
  let save = saved(`"${currentYear}"`)
  await yearSelect.selectOption({ label: currentYearLabel })
  expect((await save).ok()).toBe(true)
  save = saved(`["Book"]`)
  await checkboxes.nth(1).uncheck()
  expect((await save).ok()).toBe(true)

  // After a reload both filters are restored from the saved preferences
  await authenticatedPage.reload()
  await authenticatedPage.locator("main button").filter({ hasNotText: "Lisää uusi" }).click()
  await expect(yearSelect.locator("option:checked")).toHaveText(currentYearLabel)
  await expect(checkboxes.nth(0)).toBeChecked()
  await expect(checkboxes.nth(1)).not.toBeChecked()
})
