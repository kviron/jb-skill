import type { ParentProps } from "solid-js";

import { cx } from "../../../styled-system/css";
import { statusBar } from "../../../styled-system/recipes";

type Props = ParentProps<{ class?: string }>;

export function UiStatusBar(props: Props) {
  return <footer class={cx(statusBar(), props.class)}>{props.children}</footer>;
}
