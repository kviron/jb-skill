import { For } from "solid-js";
import { css, cx } from "../../../styled-system/css";
import { panel, panelHeader } from "../../../styled-system/recipes";

type Props = {
  operationLog: string[];
};

export function OperationsPage(props: Props) {
  const titleClass = css({ m: "0 0 2", fontSize: "lg", fontWeight: "semibold" });
  const subtitleClass = css({ m: "0 0 4", color: "text.secondary", fontSize: "sm" });
  const listClass = css({ listStyle: "none", p: "0", m: "0", display: "grid", gap: "2" });
  const itemClass = css({ fontFamily: "mono", fontSize: "sm" });

  return (
    <>
      <h2 class={titleClass}>Operation Log</h2>
      <p class={subtitleClass}>Журнал операций core-команд и фоновых событий</p>
      <ul class={listClass} role="log" aria-label="Журнал операций">
        <For each={props.operationLog}>
          {(entry) => (
            <li class={cx(panel(), itemClass)}>
              <div class={panelHeader()}>Event</div>
              {entry}
            </li>
          )}
        </For>
      </ul>
    </>
  );
}
