import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./e2e",
  use: {
    baseURL: "http://localhost:1420",
    headless: true,
  },
  webServer: {
    command: "npm run dev -- --host 0.0.0.0 --port 1420",
    port: 1420,
    reuseExistingServer: true,
    timeout: 120000,
  },
});
