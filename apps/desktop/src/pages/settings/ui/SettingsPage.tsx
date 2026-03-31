import { Show, createMemo, createSignal } from "solid-js";
import { useTranslation } from "solid-i18next";

import { useTheme } from "../../../app/providers/theme";
import { LanguageSwitch } from "../../../features";
import {
  SettingsCategoryNav,
  SettingsLayout,
  SettingsMain,
  SettingsRow,
  SettingsSearchField,
  SettingsSection,
  SettingsSidebar,
  UiButton,
  UiBadge,
} from "../../../shared/ui";
import { css } from "../../../styled-system/css";

type SettingsCategoryId = "appearance" | "editor" | "mods";

const titleClass = css({
  m: "0",
  fontSize: "lg",
  fontWeight: "semibold",
});

const subtitleClass = css({
  m: "0",
  color: "text.secondary",
  fontSize: "sm",
});

const headerClass = css({
  display: "grid",
  gap: "2",
});

const controlClass = css({
  display: "inline-flex",
  alignItems: "center",
  gap: "2",
  flexWrap: "wrap",
});

const emptyClass = css({
  m: "0",
  color: "text.secondary",
  fontSize: "sm",
});

export function SettingsPage() {
  const [t] = useTranslation(["settings", "common"]);
  const { theme, setTheme } = useTheme();

  const [activeCategory, setActiveCategory] = createSignal<SettingsCategoryId>("appearance");
  const [query, setQuery] = createSignal("");

  const categories = createMemo(() => [
    { id: "appearance" as const, label: t("settings:categoryAppearance") },
    { id: "editor" as const, label: t("settings:categoryEditor") },
    { id: "mods" as const, label: t("settings:categoryMods") },
  ]);

  const normalizedQuery = createMemo(() => query().trim().toLowerCase());
  const themeMatches = createMemo(() => {
    const q = normalizedQuery();
    if (!q) return true;
    return (
      t("settings:themeLabel").toLowerCase().includes(q) ||
      t("settings:themeDescription").toLowerCase().includes(q)
    );
  });

  const languageMatches = createMemo(() => {
    const q = normalizedQuery();
    if (!q) return true;
    return (
      t("settings:languageLabel").toLowerCase().includes(q) ||
      t("settings:languageDescription").toLowerCase().includes(q)
    );
  });

  return (
    <SettingsLayout>
      <SettingsSidebar>
        <SettingsCategoryNav
          items={categories()}
          activeId={activeCategory()}
          onSelect={(id) => setActiveCategory(id as SettingsCategoryId)}
        />
      </SettingsSidebar>
      <SettingsMain>
        <div class={headerClass}>
          <h2 class={titleClass}>{t("settings:title")}</h2>
          <p class={subtitleClass}>{t("settings:pageDescription")}</p>
        </div>

        <SettingsSearchField
          value={query()}
          onInput={setQuery}
          placeholder={t("settings:searchPlaceholder")}
          ariaLabel={t("settings:searchAria")}
        />

        <Show when={activeCategory() === "appearance"}>
          <SettingsSection
            title={t("settings:appearanceSectionTitle")}
            description={t("settings:appearanceSectionDescription")}
          >
            <Show when={themeMatches()}>
              <SettingsRow
                borderless
                label={t("settings:themeLabel")}
                description={t("settings:themeDescription")}
              >
                <div class={controlClass}>
                  <UiButton
                    variant={theme() === "dark" ? "solid" : "subtle"}
                    data-testid="settings.theme-dark"
                    onClick={() => setTheme("dark")}
                  >
                    {t("settings:darkTab")}
                  </UiButton>
                  <UiButton
                    variant={theme() === "light" ? "solid" : "subtle"}
                    data-testid="settings.theme-light"
                    onClick={() => setTheme("light")}
                  >
                    {t("settings:lightTab")}
                  </UiButton>
                  <UiBadge tone="info">
                    {theme() === "dark" ? t("common:themeDark") : t("common:themeLight")}
                  </UiBadge>
                </div>
              </SettingsRow>
            </Show>

            <Show when={languageMatches()}>
              <SettingsRow
                label={t("settings:languageLabel")}
                description={t("settings:languageDescription")}
              >
                <LanguageSwitch />
              </SettingsRow>
            </Show>

            <Show when={!themeMatches() && !languageMatches()}>
              <div class={css({ px: "3", py: "3" })}>
                <p class={emptyClass}>{t("settings:noSettingsFound")}</p>
              </div>
            </Show>
          </SettingsSection>
        </Show>

        <Show when={activeCategory() === "editor" || activeCategory() === "mods"}>
          <SettingsSection
            title={t("settings:placeholderSectionTitle")}
            description={t("settings:placeholderSectionDescription")}
          >
            <div class={css({ px: "3", py: "3" })}>
              <p class={emptyClass}>{t("settings:placeholderSectionMessage")}</p>
            </div>
          </SettingsSection>
        </Show>
      </SettingsMain>
    </SettingsLayout>
  );
}
