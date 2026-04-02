import { For, Show, createSignal } from "solid-js";
import { createVirtualizer } from "@tanstack/solid-virtual";
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
  reorderingModId: string | null;
  onArchivePathInput: (value: string) => void;
  onSearchInput: (value: string) => void;
  onInstall: () => void;
  onToggle: (item: ModRecord) => void;
  onRemove: (item: ModRecord) => void;
  onPickArchive: () => void;
  onReorderUp: (item: ModRecord) => void;
  onReorderDown: (item: ModRecord) => void;
};

export function ModsPage(props: Props) {
  const [t] = useTranslation(["common", "pages"]);
  const [scrollParent, setScrollParent] = createSignal<HTMLDivElement | null>(null);

  const virtualizer = createVirtualizer(() => ({
    count: props.mods.length,
    getScrollElement: () => scrollParent() ?? null,
    estimateSize: () => 112,
    overscan: 8,
  }));

  const titleClass = css({ m: "0 0 2", fontSize: "lg", fontWeight: "semibold" });
  const subtitleClass = css({ m: "0 0 4", color: "text.secondary", fontSize: "sm" });
  const formClass = css({
    display: "flex",
    flexDirection: "column",
    gap: "3",
    mb: "4",
    maxW: "960px",
  });
  const pathRowClass = css({
    display: "flex",
    gap: "3",
    flexWrap: "wrap",
    alignItems: "center",
  });
  const pathInputClass = css({ flex: "1", minW: "200px" });
  const labelClass = css({ color: "text.secondary", fontSize: "sm" });
  const searchWrapClass = css({ display: "grid", gap: "2", mb: "4", maxW: "640px" });
  const listScrollClass = css({
    minH: "0",
    maxH: "min(60vh, 560px)",
    overflowY: "auto",
    borderWidth: "1px",
    borderStyle: "solid",
    borderColor: "border.strong",
    borderRadius: "md",
  });
  const listInnerClass = css({ position: "relative", w: "100%" });
  const rowClass = css({
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    gap: "3",
    px: "3",
    py: "3",
    boxSizing: "border-box",
  });
  const itemMetaClass = css({ display: "grid", gap: "1", minW: "0" });
  const actionsClass = css({ display: "flex", gap: "2", flexWrap: "wrap", flexShrink: "0" });

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
        <label class={labelClass} for="archivePath">
          {t("pages:modsArchiveLabel")}
        </label>
        <div class={pathRowClass}>
          <UiInput
            id="archivePath"
            class={pathInputClass}
            data-testid="mods.archive-input"
            value={props.modArchivePath}
            onInput={(event) => props.onArchivePathInput(event.currentTarget.value)}
            placeholder="C:/mods/my-mod.zip"
          />
          <UiButton type="button" variant="outline" onClick={() => props.onPickArchive()}>
            {t("pages:modsBrowse")}
          </UiButton>
          <UiButton variant="solid" type="submit" data-testid="mods.install-button" disabled={props.installing}>
            {props.installing ? t("common:installing") : t("common:install")}
          </UiButton>
        </div>
      </form>

      <div class={searchWrapClass}>
        <label class={labelClass} for="searchInput">
          {t("pages:modsSearchLabel")}
        </label>
        <UiInput
          id="searchInput"
          data-testid="mods.search-input"
          value={props.modSearch}
          onInput={(event) => props.onSearchInput(event.currentTarget.value)}
          placeholder={t("pages:modsSearchPlaceholder")}
        />
      </div>

      <div class={listScrollClass} ref={setScrollParent}>
        <div class={listInnerClass} style={{ height: `${virtualizer.getTotalSize()}px` }}>
          <For each={virtualizer.getVirtualItems()}>
            {(virtualRow) => {
              const item = () => props.mods[virtualRow.index];
              return (
                <div
                  class={cx(panel(), rowClass)}
                  style={{
                    position: "absolute",
                    top: "0",
                    left: "0",
                    width: "100%",
                    transform: `translateY(${virtualRow.start}px)`,
                  }}
                >
                  <div class={panelHeader()}>{t("pages:modEntry")}</div>
                  <div class={itemMetaClass}>
                    <strong>{item().name}</strong>
                    <UiBadge tone={item().enabled ? "success" : "warning"}>
                      {item().enabled
                        ? t("common:enabledVersion", { version: item().version ?? "0.0.0" })
                        : t("common:disabledVersion", { version: item().version ?? "0.0.0" })}
                    </UiBadge>
                  </div>
                  <div class={actionsClass}>
                    <UiButton
                      type="button"
                      disabled={props.reorderingModId === item().id || virtualRow.index === 0}
                      onClick={() => props.onReorderUp(item())}
                    >
                      {t("pages:modMoveUp")}
                    </UiButton>
                    <UiButton
                      type="button"
                      disabled={
                        props.reorderingModId === item().id || virtualRow.index === props.mods.length - 1
                      }
                      onClick={() => props.onReorderDown(item())}
                    >
                      {t("pages:modMoveDown")}
                    </UiButton>
                    <UiButton
                      data-testid={`mods.toggle.${item().id}`}
                      disabled={props.togglingModId === item().id}
                      onClick={() => props.onToggle(item())}
                    >
                      {item().enabled ? t("common:disable") : t("common:enable")}
                    </UiButton>
                    <UiButton
                      variant="danger"
                      data-testid={`mods.remove.${item().id}`}
                      disabled={props.removingModId === item().id}
                      onClick={() => props.onRemove(item())}
                    >
                      {t("common:remove")}
                    </UiButton>
                  </div>
                </div>
              );
            }}
          </For>
        </div>
      </div>
      <Show when={props.mods.length === 0}>
        <p class={css({ color: "text.secondary", fontSize: "sm" })}>{t("pages:modsEmpty")}</p>
      </Show>
    </>
  );
}
