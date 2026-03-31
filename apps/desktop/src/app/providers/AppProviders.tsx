import type { ParentProps } from "solid-js";
import { ThemeProvider } from "./theme";

export function AppProviders(props: ParentProps) {
  return <ThemeProvider>{props.children}</ThemeProvider>;
}
