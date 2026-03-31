import type { JSX } from "solid-js";

import { cx } from "../../../styled-system/css";
import { input } from "../../../styled-system/recipes";

type Props = {
  class?: string;
} & JSX.InputHTMLAttributes<HTMLInputElement>;

export function UiInput(props: Props) {
  return <input {...props} class={cx(input(), props.class)} />;
}
