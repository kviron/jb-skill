import { defineConfig } from "@pandacss/dev";
import { hiTechRecipes, hiTechSemanticTokens, hiTechTokens } from "./src/shared/styles/panda";

export default defineConfig({
  preflight: true,
  include: ["./src/**/*.{ts,tsx,js,jsx}"],
  exclude: ["./src/**/*.test.{ts,tsx}", "./src/**/*.spec.{ts,tsx}"],
  outdir: "src/styled-system",
  jsxFramework: "solid",
  hash: false,
  minify: true,
  conditions: {
    light: "&:where([data-theme=light], [data-theme=light] *)",
    dark: "&:where([data-theme=dark], [data-theme=dark] *)",
  },
  theme: {
    extend: {
      tokens: hiTechTokens,
      semanticTokens: hiTechSemanticTokens,
      recipes: hiTechRecipes,
    },
  },
  globalCss: {
    "html, body, #root": {
      minHeight: "100%",
    },
    body: {
      margin: "0",
      bg: "bg.canvas",
      color: "text.primary",
      fontFamily: "sans",
    },
    "*": {
      boxSizing: "border-box",
    },
    "*:focus-visible": {
      outline: "2px solid token(colors.focus.ring)",
      outlineOffset: "2px",
    },
    "@media (prefers-reduced-motion: reduce)": {
      "*, *::before, *::after": {
        animationDuration: "0.01ms !important",
        animationIterationCount: "1 !important",
        transitionDuration: "0.01ms !important",
        scrollBehavior: "auto !important",
      },
    },
  },
});
