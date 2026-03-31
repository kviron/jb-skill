import type { JSX, ParentProps } from "solid-js";

import { cx } from "../../../styled-system/css";
import { panel } from "../../../styled-system/recipes";

type Props = ParentProps<{ class?: string } & JSX.HTMLAttributes<HTMLElement>>;

export function UiPanel(props: Props) {
  return <section class={cx(panel(), props.class)}>{props.children}</section>;
}
