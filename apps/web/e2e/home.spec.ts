import { test } from "@playwright/test";

test.describe("Home", () => {
  test("should display the correct content", async ({ page }) => {
    await page.goto("/");
  });
});
