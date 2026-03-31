import { For, Show } from "solid-js";
import { useTranslation } from "solid-i18next";

import type { ConflictRecord } from "../../../shared/api/core";
import { css, cx } from "../../../styled-system/css";
import { panel, panelHeader } from "../../../styled-system/recipes";
import { UiBadge } from "../../../shared/ui/primitives";

type Props = {
  conflicts: ConflictRecord[];
};

export function ConflictsPage(props: Props) {
  const [t] = useTranslation(["common", "pages"]);
  const titleClass = css({ m: "0 0 2", fontSize: "lg", fontWeight: "semibold" });
  const subtitleClass = css({ m: "0 0 4", color: "text.secondary", fontSize: "sm" });
  const listClass = css({ listStyle: "none", p: "0", m: "0", display: "grid", gap: "3" });
  const itemClass = css({ display: "grid", gap: "2" });
  const rowClass = css({ display: "flex", gap: "2", flexWrap: "wrap", alignItems: "center" });

  return (
    <>
      <h2 class={titleClass}>{t("pages:conflictsTitle")}</h2>
      <p class={subtitleClass}>{t("pages:conflictsSubtitle")}</p>
      <Show when={props.conflicts.length > 0} fallback={<p>{t("common:noConflictsYet")}</p>}>
        <ul class={listClass}>
          <For each={props.conflicts}>
            {(item) => (
              <li class={cx(panel(), itemClass)}>
                <div class={panelHeader()}>{t("pages:conflictEntry")}</div>
                <strong>{item.targetPath}</strong>
                <div class={rowClass}>
                  <UiBadge tone="success">{t("pages:winner")}: {item.winnerModId}</UiBadge>
                  <UiBadge tone="warning">{t("pages:losers")}: {item.loserModIdsJson}</UiBadge>
                </div>
              </li>
            )}
          </For>
        </ul>
      </Show>
    </>
  );
}
