import type { ParentProps } from "solid-js";

import { css } from "../../../styled-system/css";

const layoutClass = css({
  display: "grid",
  gridTemplateColumns: "220px minmax(0, 1fr)",
  minH: "100%",
  borderWidth: "1px",
  borderStyle: "solid",
  borderColor: "border.strong",
  bg: "bg.surface",
});

const sidebarClass = css({
  borderRightWidth: "1px",
  borderRightStyle: "solid",
  borderRightColor: "border.strong",
  bg: "bg.sidebar",
  p: "2",
  minH: "0",
});

const mainClass = css({
  p: "4",
  minW: "0",
  minH: "0",
  overflow: "auto",
  display: "grid",
  gap: "4",
  alignContent: "start",
});

export function SettingsLayout(props: ParentProps) {
  return <div class={layoutClass}>{props.children}</div>;
}

export function SettingsSidebar(props: ParentProps) {
  return <aside class={sidebarClass}>{props.children}</aside>;
}

export function SettingsMain(props: ParentProps) {
  return <section class={mainClass}>{props.children}</section>;
}
