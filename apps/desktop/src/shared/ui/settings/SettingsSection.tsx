import type { ParentProps } from "solid-js";

import { css } from "../../../styled-system/css";

type Props = ParentProps<{
  title: string;
  description?: string;
}>;

const sectionClass = css({
  display: "grid",
  gap: "2",
});

const titleClass = css({
  m: "0",
  fontSize: "sm",
  fontWeight: "semibold",
  textTransform: "uppercase",
  letterSpacing: "0.04em",
  color: "text.muted",
});

const descriptionClass = css({
  m: "0",
  color: "text.secondary",
  fontSize: "sm",
});

const contentClass = css({
  display: "grid",
  borderWidth: "1px",
  borderStyle: "solid",
  borderColor: "border.default",
});

export function SettingsSection(props: Props) {
  return (
    <section class={sectionClass}>
      <h3 class={titleClass}>{props.title}</h3>
      {props.description ? <p class={descriptionClass}>{props.description}</p> : null}
      <div class={contentClass}>{props.children}</div>
    </section>
  );
}
