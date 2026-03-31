import type { JSX, ParentProps } from "solid-js";

import { cx } from "../../../styled-system/css";
import { sideNavItem } from "../../../styled-system/recipes";

type Props = ParentProps<{
  active?: boolean;
  collapsed?: boolean;
  class?: string;
} & JSX.ButtonHTMLAttributes<HTMLButtonElement>>;

export function UiSideNavItem(props: Props) {
  return (
    <button
      {...props}
      class={cx(
        sideNavItem({
          active: props.active ?? false,
          collapsed: props.collapsed ?? false,
        }),
        props.class,
      )}
    >
      {props.children}
    </button>
  );
}
