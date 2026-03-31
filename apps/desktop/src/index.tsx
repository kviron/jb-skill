/* @refresh reload */
import { render } from "solid-js/web";
import { AppProviders } from "./app/providers/AppProviders";
import { AppRouter } from "./app/router/AppRouter";
import "./shared/i18n/config";
import "./styled-system/styles.css";

render(
  () => (
    <AppProviders>
      <AppRouter />
    </AppProviders>
  ),
  document.getElementById("root") as HTMLElement,
);
