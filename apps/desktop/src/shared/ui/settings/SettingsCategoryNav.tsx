import { For } from "solid-js";

import { css, cx } from "../../../styled-system/css";

type Item = {
  id: string;
  label: string;
};

type Props = {
  items: Item[];
  activeId: string;
  onSelect: (id: string) => void;
};

const listClass = css({
  listStyle: "none",
  p: "0",
  m: "0",
  display: "grid",
  gap: "1",
});

const itemButtonClass = css({
  w: "100%",
  minH: "touch",
  px: "3",
  borderWidth: "1px",
  borderStyle: "solid",
  borderColor: "transparent",
  bg: "transparent",
  color: "text.secondary",
  textAlign: "left",
  fontSize: "sm",
  cursor: "pointer",
  _hover: {
    bg: "bg.hover",
    color: "text.primary",
    borderColor: "border.default",
  },
  _focusVisible: {
    outline: "2px solid token(colors.focus.ring)",
    outlineOffset: "1px",
  },
});

const itemButtonActiveClass = css({
  bg: "bg.selected",
  color: "text.primary",
  borderColor: "border.default",
  borderLeftColor: "accent.default",
  borderLeftWidth: "2px",
});

export function SettingsCategoryNav(props: Props) {
  return (
    <ul class={listClass}>
      <For each={props.items}>
        {(item) => (
          <li>
            <button
              type="button"
              class={cx(itemButtonClass, props.activeId === item.id ? itemButtonActiveClass : "")}
              onClick={() => props.onSelect(item.id)}
            >
              {item.label}
            </button>
          </li>
        )}
      </For>
    </ul>
  );
}
