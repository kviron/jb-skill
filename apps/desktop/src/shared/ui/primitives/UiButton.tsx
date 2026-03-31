import type { JSX } from "solid-js";

import { cx } from "../../../styled-system/css";
import { button } from "../../../styled-system/recipes";

type Variant = "solid" | "subtle" | "danger";

type Props = {
  variant?: Variant;
  class?: string;
} & JSX.ButtonHTMLAttributes<HTMLButtonElement>;

export function UiButton(props: Props) {
  return (
    <button
      {...props}
      class={cx(button({ variant: props.variant ?? "subtle" }), props.class)}
    />
  );
}
