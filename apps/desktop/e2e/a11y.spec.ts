import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test("a11y: shell baseline has no critical violations", async ({ page }) => {
  await page.goto("http://localhost:1420");
  const results = await new AxeBuilder({ page }).analyze();
  const criticalViolations = results.violations.filter((violation) => violation.impact === "critical");
  expect(criticalViolations).toEqual([]);
});

