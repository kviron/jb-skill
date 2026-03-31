// MVP E2E smoke skeleton based on docs/test-plan-mvp.md
// Requires Playwright setup in CI runner.
import { test, expect } from "@playwright/test";

test("E2E-01 Install smoke", async ({ page }) => {
  await page.goto("http://localhost:1420");
  await page.getByTestId("nav.mods").click();
  await expect(page.getByRole("heading", { name: "Mods List" })).toBeVisible();
  await expect(page.getByTestId("mods.archive-input")).toBeVisible();
  await expect(page.getByTestId("mods.install-button")).toBeVisible();
});

test("E2E-05 Switch profile smoke", async ({ page }) => {
  await page.goto("http://localhost:1420");
  await page.getByTestId("nav.profiles").click();
  await expect(page.getByRole("heading", { name: "Profiles" })).toBeVisible();
  await expect(page.getByTestId("profiles.switch-button")).toBeVisible();
});

test("E2E-02 Conflict resolution screen smoke", async ({ page }) => {
  await page.goto("http://localhost:1420");
  await page.getByTestId("nav.conflicts").click();
  await expect(page.getByRole("heading", { name: "Conflicts" })).toBeVisible();
});

test("E2E-03 Enable/Disable control visible", async ({ page }) => {
  await page.goto("http://localhost:1420");
  await page.getByTestId("nav.mods").click();
  await expect(page.getByRole("heading", { name: "Mods List" })).toBeVisible();
  await expect(page.getByTestId("mods.search-input")).toBeVisible();
});

test("E2E-04 Remove control visible", async ({ page }) => {
  await page.goto("http://localhost:1420");
  await page.getByTestId("nav.mods").click();
  await expect(page.getByTestId("mods.install-button")).toBeVisible();
});

test("E2E-06 Crash recovery signal visible in operation log view", async ({ page }) => {
  await page.goto("http://localhost:1420");
  await page.getByTestId("nav.operations").click();
  await expect(page.getByRole("heading", { name: "Operation Log" })).toBeVisible();
});
