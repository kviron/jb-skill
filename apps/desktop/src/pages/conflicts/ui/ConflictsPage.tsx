import { For, Show } from "solid-js";

import type { ConflictRecord } from "../../../shared/api/core";
import { css, cx } from "../../../styled-system/css";
import { panel, panelHeader } from "../../../styled-system/recipes";
import { UiBadge } from "../../../shared/ui/primitives";

type Props = {
  conflicts: ConflictRecord[];
};

export function ConflictsPage(props: Props) {
  const titleClass = css({ m: "0 0 2", fontSize: "lg", fontWeight: "semibold" });
  const subtitleClass = css({ m: "0 0 4", color: "text.secondary", fontSize: "sm" });
  const listClass = css({ listStyle: "none", p: "0", m: "0", display: "grid", gap: "3" });
  const itemClass = css({ display: "grid", gap: "2" });
  const rowClass = css({ display: "flex", gap: "2", flexWrap: "wrap", alignItems: "center" });

  return (
    <>
      <h2 class={titleClass}>Conflicts</h2>
      <p class={subtitleClass}>Диагностика конфликтов и порядок разрешения winner/losers</p>
      <Show when={props.conflicts.length > 0} fallback={<p>Конфликтов пока нет</p>}>
        <ul class={listClass}>
          <For each={props.conflicts}>
            {(item) => (
              <li class={cx(panel(), itemClass)}>
                <div class={panelHeader()}>Conflict Entry</div>
                <strong>{item.targetPath}</strong>
                <div class={rowClass}>
                  <UiBadge tone="success">Winner: {item.winnerModId}</UiBadge>
                  <UiBadge tone="warning">Losers: {item.loserModIdsJson}</UiBadge>
                </div>
              </li>
            )}
          </For>
        </ul>
      </Show>
    </>
  );
}
