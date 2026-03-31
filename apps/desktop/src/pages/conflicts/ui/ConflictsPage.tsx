import { For, Show } from "solid-js";

import type { ConflictRecord } from "../../../shared/api/core";

type Props = {
  conflicts: ConflictRecord[];
};

export function ConflictsPage(props: Props) {
  return (
    <>
      <h2>Conflicts</h2>
      <Show when={props.conflicts.length > 0} fallback={<p>Конфликтов пока нет</p>}>
        <ul class="conflicts-list">
          <For each={props.conflicts}>
            {(item) => (
              <li>
                <strong>{item.targetPath}</strong>
                <span>Winner: {item.winnerModId}</span>
                <span>Losers: {item.loserModIdsJson}</span>
              </li>
            )}
          </For>
        </ul>
      </Show>
    </>
  );
}
