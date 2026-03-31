/* @refresh reload */
import { render } from "solid-js/web";
import { AppProviders } from "./app/providers/AppProviders";
import { AppRouter } from "./app/router/AppRouter";
import "./styled-system/styles.css";

render(
  () => (
    <AppProviders>
      <AppRouter />
    </AppProviders>
  ),
  document.getElementById("root") as HTMLElement,
);
