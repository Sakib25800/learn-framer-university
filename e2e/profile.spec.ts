import { expect, test } from "@playwright/test"

test.describe("Profile Page", () => {
  test.use({ storageState: "playwright/.auth/user.json" })

  test.beforeEach(async ({ page }) => {
    await page.goto("/profile")
  })

  test("should show user profile and email", async ({ page }) => {
    await expect(page.locator("pre")).toContainText("test@example.com")
  })

  test("should sign out user", async ({ page }) => {
    await page.getByText("Sign out").click()
    await expect(page).toHaveURL("/sign-in")
  })
})
