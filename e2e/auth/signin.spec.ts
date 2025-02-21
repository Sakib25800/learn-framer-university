import { test, expect } from "@playwright/test"
import { readLatestEmail, extractContinueToken } from "../helpers/email"

test.describe("Sign-in flow", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/sign-in")
  })

  test("shows validation for invalid email", async ({ page }) => {
    await page.fill('input[name="email"]', "invalidemail@asd")
    await page.click("button[type='submit']")
    await expect(page.getByText(/email/i)).toBeVisible()
  })

  test("completes sign-in flow successfully", async ({ page }) => {
    await page.fill('input[name="email"]', "test@example.com")
    await page.click("button[type='submit']")

    // Ensure some sort of success message is shown
    await expect(page.getByTestId("success-message")).toBeVisible()

    // Retrieve continue token from email
    const emailContent = await readLatestEmail()
    expect(emailContent).toBeTruthy()

    const continueToken = extractContinueToken(emailContent!)
    expect(continueToken).toBeTruthy()

    // Complete sign-in
    await page.goto(`/api/continue/${continueToken}`)
    await expect(page).toHaveURL("/profile")
  })

  test("handles invalid continue token", async ({ page }) => {
    await page.goto("/api/continue/invalid-token")
    await expect(page).toHaveURL(/\/sign-in\?error=/)
  })
})
