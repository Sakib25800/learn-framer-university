import { test, expect } from "@playwright/test";

test("check for text on the page", async ({ page }) => {
    await page.goto("/");
    const content = await page.content();
    expect(content).toContain("Get started");
});
