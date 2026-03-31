import type { ParentProps } from "solid-js";

import { css } from "../../../styled-system/css";

type Props = ParentProps<{
  label: string;
  description?: string;
  borderless?: boolean;
}>;

const rowClass = css({
  display: "grid",
  gridTemplateColumns: "minmax(0, 1fr) auto",
  alignItems: "center",
  gap: "3",
  px: "3",
  py: "3",
  borderTopWidth: "1px",
  borderTopStyle: "solid",
  borderTopColor: "border.default",
  _first: {
    borderTopWidth: "0",
  },
});

const rowBorderlessClass = css({
  borderTopWidth: "0",
});

const textWrapClass = css({
  display: "grid",
  gap: "1",
  minW: "0",
});

const labelClass = css({
  fontSize: "sm",
  fontWeight: "medium",
  color: "text.primary",
});

const descriptionClass = css({
  m: "0",
  fontSize: "sm",
  color: "text.secondary",
});

const controlClass = css({
  display: "inline-flex",
  alignItems: "center",
  gap: "2",
  flexWrap: "wrap",
  justifySelf: "end",
});

export function SettingsRow(props: Props) {
  return (
    <div class={props.borderless ? `${rowClass} ${rowBorderlessClass}` : rowClass}>
      <div class={textWrapClass}>
        <span class={labelClass}>{props.label}</span>
        {props.description ? <p class={descriptionClass}>{props.description}</p> : null}
      </div>
      <div class={controlClass}>{props.children}</div>
    </div>
  );
}
