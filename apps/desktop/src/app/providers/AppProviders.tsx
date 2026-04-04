import type { ParentProps } from "solid-js";

import { LanguageProvider } from "../../shared/i18n";

import { ThemeProvider } from "./theme";

export function AppProviders(props: ParentProps) {
  return (
    <LanguageProvider>
      <ThemeProvider>{props.children}</ThemeProvider>
    </LanguageProvider>
  );
}
