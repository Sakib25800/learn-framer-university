import { expect, test } from "@playwright/test"

test.describe("Profile page", () => {
  test.describe("Authenticated user", () => {
    test.use({ storageState: "playwright/.auth/user.json" })

    test.beforeEach(async ({ page }) => {
      await page.goto("/profile")
    })

    test("displays user information", async ({ page }) => {
      await expect(page.locator("h1")).toHaveText("Profile")
      await expect(page.locator("pre")).toContainText("test@example.com")
      await expect(page.getByRole("button", { name: /sign out/i })).toBeVisible()
    })

    test("handles sign out", async ({ page }) => {
      await page.getByText("Sign out").click()
      await expect(page).toHaveURL("/sign-in")

      // Verify profile access is restricted
      await page.goto("/profile")
      await expect(page).toHaveURL(/\/sign-in/)
    })
  })

  test.describe("Unauthenticated user", () => {
    test("redirects to sign in", async ({ page }) => {
      await page.goto("/profile")
      await expect(page).toHaveURL(/\/sign-in/)
    })
  })
})
