import { For } from "solid-js";
import { useTranslation } from "solid-i18next";

import type { ModRecord } from "../../../shared/api/core";
import { css, cx } from "../../../styled-system/css";
import { panel, panelHeader } from "../../../styled-system/recipes";
import { UiBadge, UiButton, UiInput } from "../../../shared/ui/primitives";

type Props = {
  modArchivePath: string;
  modSearch: string;
  mods: ModRecord[];
  installing: boolean;
  togglingModId: string | null;
  removingModId: string | null;
  onArchivePathInput: (value: string) => void;
  onSearchInput: (value: string) => void;
  onInstall: () => void;
  onToggle: (item: ModRecord) => void;
  onRemove: (item: ModRecord) => void;
};

export function ModsPage(props: Props) {
  const [t] = useTranslation(["common", "pages"]);
  const titleClass = css({ m: "0 0 2", fontSize: "lg", fontWeight: "semibold" });
  const subtitleClass = css({ m: "0 0 4", color: "text.secondary", fontSize: "sm" });
  const formClass = css({
    display: "grid",
    gap: "3",
    gridTemplateColumns: "180px 1fr auto",
    alignItems: "end",
    mb: "4",
  });
  const labelClass = css({ color: "text.secondary", fontSize: "sm" });
  const searchWrapClass = css({ display: "grid", gap: "2", mb: "4", maxW: "640px" });
  const listClass = css({ listStyle: "none", p: "0", m: "0", display: "grid", gap: "3" });
  const itemClass = css({
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    gap: "3",
  });
  const itemMetaClass = css({ display: "grid", gap: "1" });
  const actionsClass = css({ display: "flex", gap: "2", flexWrap: "wrap" });

  return (
    <>
      <h2 class={titleClass}>{t("pages:modsTitle")}</h2>
      <p class={subtitleClass}>{t("pages:modsSubtitle")}</p>
      <form
        class={formClass}
        onSubmit={(event) => {
          event.preventDefault();
          props.onInstall();
        }}
      >
        <label class={labelClass} for="archivePath">{t("pages:modsArchiveLabel")}</label>
        <UiInput
          id="archivePath"
          data-testid="mods.archive-input"
          value={props.modArchivePath}
          onInput={(event) => props.onArchivePathInput(event.currentTarget.value)}
          placeholder="C:/mods/my-mod.zip"
        />
        <UiButton variant="solid" type="submit" data-testid="mods.install-button" disabled={props.installing}>
          {props.installing ? t("common:installing") : t("common:install")}
        </UiButton>
      </form>

      <div class={searchWrapClass}>
        <label class={labelClass} for="searchInput">{t("pages:modsSearchLabel")}</label>
        <UiInput
          id="searchInput"
          data-testid="mods.search-input"
          value={props.modSearch}
          onInput={(event) => props.onSearchInput(event.currentTarget.value)}
          placeholder={t("pages:modsSearchPlaceholder")}
        />
      </div>

      <ul class={listClass}>
        <For each={props.mods}>
          {(item) => (
            <li class={cx(panel(), itemClass)}>
              <div class={panelHeader()}>{t("pages:modEntry")}</div>
              <div class={itemMetaClass}>
                <strong>{item.name}</strong>
                <UiBadge tone={item.enabled ? "success" : "warning"}>
                  {item.enabled
                    ? t("common:enabledVersion", { version: item.version ?? "0.0.0" })
                    : t("common:disabledVersion", { version: item.version ?? "0.0.0" })}
                </UiBadge>
              </div>
              <div class={actionsClass}>
                <UiButton
                  data-testid={`mods.toggle.${item.id}`}
                  disabled={props.togglingModId === item.id}
                  onClick={() => props.onToggle(item)}
                >
                  {item.enabled ? t("common:disable") : t("common:enable")}
                </UiButton>
                <UiButton
                  variant="danger"
                  data-testid={`mods.remove.${item.id}`}
                  disabled={props.removingModId === item.id}
                  onClick={() => props.onRemove(item)}
                >
                  {t("common:remove")}
                </UiButton>
              </div>
            </li>
          )}
        </For>
      </ul>
    </>
  );
}
