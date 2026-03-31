import { createMemo } from "solid-js";
import { useTranslation } from "solid-i18next";

import { useLanguage } from "../../../app/providers/language";
import type { AppLanguage } from "../../../shared/i18n";
import { css } from "../../../styled-system/css";
import { UiButton } from "../../../shared/ui/primitives";

type Props = {
  testIdPrefix?: string;
};

export function LanguageSwitch(props: Props) {
  const [t] = useTranslation();
  const { language, setLanguage } = useLanguage();

  const rootClass = css({
    display: "flex",
    alignItems: "center",
    gap: "2",
    flexWrap: "wrap",
  });

  const currentLanguage = createMemo(() => language());

  const handleSelect = async (nextLanguage: AppLanguage) => {
    await setLanguage(nextLanguage);
  };

  const testIdPrefix = () => props.testIdPrefix ?? "settings.language";

  return (
    <div class={rootClass} role="group" aria-label={t("settings:languageSwitcherAria")}>
      <UiButton
        variant={currentLanguage() === "ru" ? "solid" : "subtle"}
        data-testid={`${testIdPrefix()}.ru`}
        onClick={[handleSelect, "ru"]}
      >
        {t("settings:languageRu")}
      </UiButton>
      <UiButton
        variant={currentLanguage() === "en" ? "solid" : "subtle"}
        data-testid={`${testIdPrefix()}.en`}
        onClick={[handleSelect, "en"]}
      >
        {t("settings:languageEn")}
      </UiButton>
    </div>
  );
}
