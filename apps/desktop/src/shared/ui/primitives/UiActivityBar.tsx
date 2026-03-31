import type { ParentProps } from "solid-js";

import { css, cx } from "../../../styled-system/css";

type Props = ParentProps<{ class?: string }>;

const rootClass = css({
  w: "activitybar",
  bg: "bg.activity",
  borderRightWidth: "1px",
  borderRightStyle: "solid",
  borderRightColor: "border.strong",
  display: "flex",
  flexDirection: "column",
  alignItems: "stretch",
  py: "2",
  gap: "1",
});

export function UiActivityBar(props: Props) {
  return <aside class={cx(rootClass, props.class)}>{props.children}</aside>;
}
