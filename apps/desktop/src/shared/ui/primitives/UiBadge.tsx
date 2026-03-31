import type { ParentProps } from "solid-js";

import { cx } from "../../../styled-system/css";
import { badge } from "../../../styled-system/recipes";

type Props = ParentProps<{
  tone?: "info" | "success" | "warning" | "error";
  class?: string;
}>;

export function UiBadge(props: Props) {
  return (
    <span class={cx(badge({ tone: props.tone ?? "info" }), props.class)}>
      {props.children}
    </span>
  );
}
