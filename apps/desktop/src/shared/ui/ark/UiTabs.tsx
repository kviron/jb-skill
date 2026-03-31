import { Tabs } from "@ark-ui/solid/tabs";
import type { JSX } from "solid-js";

import { css } from "../../../styled-system/css";

const listClass = css({
  display: "flex",
  gap: "2",
  borderBottomWidth: "1px",
  borderBottomStyle: "solid",
  borderBottomColor: "border.default",
  pb: "2",
});

const triggerClass = css({
  minH: "touch",
  px: "4",
  borderWidth: "1px",
  borderStyle: "solid",
  borderColor: "border.default",
  borderRadius: "none",
  bg: "bg.elevated",
  color: "text.primary",
  cursor: "pointer",
  _focusVisible: {
    outline: "2px solid token(colors.focus.ring)",
    outlineOffset: "2px",
  },
  "&[data-selected]": {
    bg: "accent.default",
    color: "text.inverse",
    borderColor: "accent.default",
  },
});

const contentClass = css({
  mt: "4",
});

type RootProps = JSX.ComponentProps<typeof Tabs.Root>;
type TriggerProps = JSX.ComponentProps<typeof Tabs.Trigger>;
type ContentProps = JSX.ComponentProps<typeof Tabs.Content>;

export function UiTabsRoot(props: RootProps) {
  return <Tabs.Root {...props} />;
}

export function UiTabsList(props: JSX.ComponentProps<typeof Tabs.List>) {
  return <Tabs.List {...props} class={listClass} />;
}

export function UiTabsTrigger(props: TriggerProps) {
  return <Tabs.Trigger {...props} class={triggerClass} />;
}

export function UiTabsContent(props: ContentProps) {
  return <Tabs.Content {...props} class={contentClass} />;
}
