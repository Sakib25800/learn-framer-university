import { test, expect } from "@playwright/test"

test.describe("Home", () => {
  test("should display the correct content", async ({ page }) => {
    await page.goto("/")

    const mainHeading = page.locator("h1")
    await expect(mainHeading).toHaveText("Framer University")

    const subHeading = page.locator("p", {
      hasText: "The world's first learning platform dedicated to teaching Framer in a fun & efficient way.",
    })
    await expect(subHeading).toBeVisible()

    const joinWaitlistButton = page.locator("button", { hasText: "Join Waitlist" })
    await expect(joinWaitlistButton).toBeVisible()

    const learnMoreButton = page.locator("button", { hasText: "Learn More" })
    await expect(learnMoreButton).toBeVisible()
  })
})
