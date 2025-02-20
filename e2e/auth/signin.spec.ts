import { test, expect } from "@playwright/test"

test.describe("Sign in flow", () => {
  test("shows error for invalid email", async ({ page }) => {
    await page.goto("/sign-in")
    await page.fill('input[name="email"]', "invalidemail@asd")
    await page.click("button[type='submit']")

    await expect(page.getByText(/email/i)).toBeVisible()
  })

  test("shows error for invalid continue token", async ({ page }) => {
    await page.goto("/api/continue/invalid-token")

    const message = encodeURIComponent("Verification failed")

    await expect(page).toHaveURL(`/sign-in?error=${message}`)
  })
})
