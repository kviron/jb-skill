import { Select, createListCollection } from "@ark-ui/solid/select";
import { For, createMemo } from "solid-js";
import { useTranslation } from "solid-i18next";
import { ChevronDown } from "lucide-solid";

import { isAppLanguage, useLanguage, type AppLanguage } from "../../../shared/i18n";
import { css } from "../../../styled-system/css";

type LangItem = { value: AppLanguage; label: string };

type Props = {
  testIdPrefix?: string;
};

const rootClass = css({
  display: "flex",
  alignItems: "center",
  minW: "36",
});

const controlClass = css({
  position: "relative",
  w: "full",
  maxW: "64",
});

const triggerClass = css({
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  gap: "2",
  w: "full",
  minH: "touch",
  px: "3",
  py: "2",
  borderWidth: "1px",
  borderStyle: "solid",
  borderColor: "border.default",
  borderRadius: "none",
  bg: "bg.elevated",
  color: "text.primary",
  cursor: "pointer",
  fontSize: "sm",
  textAlign: "left",
  transition: "background-color 0.12s ease",
  _hover: {
    bg: "bg.hover",
  },
  "&[data-state=open] svg": {
    transform: "rotate(180deg)",
  },
});

const indicatorClass = css({
  display: "flex",
  alignItems: "center",
  color: "icon.default",
  flexShrink: 0,
  "& svg": {
    transition: "transform 0.15s ease",
  },
});

const positionerClass = css({
  zIndex: 1400,
});

const contentClass = css({
  minW: "var(--reference-width)",
  bg: "bg.elevated",
  borderWidth: "1px",
  borderStyle: "solid",
  borderColor: "border.default",
  boxShadow: "0 4px 16px rgba(15, 23, 42, 0.12)",
  _dark: {
    boxShadow: "0 4px 20px rgba(0, 0, 0, 0.45)",
  },
});

const listClass = css({
  maxH: "48",
  overflowY: "auto",
  py: "1",
});

const itemClass = css({
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  gap: "2",
  minH: "touch",
  px: "3",
  py: "2",
  cursor: "pointer",
  fontSize: "sm",
  color: "text.primary",
  "&[data-highlighted]": {
    bg: "bg.hover",
  },
  "&[data-state=checked]": {
    bg: "bg.selected",
  },
});

const itemIndicatorClass = css({
  display: "flex",
  alignItems: "center",
  color: "accent.default",
  fontSize: "xs",
});

export function LanguageSwitch(props: Props) {
  const [t] = useTranslation();
  const { language, setLanguage } = useLanguage();

  const collection = createMemo(() => {
    language();
    return createListCollection<LangItem>({
      items: [
        { value: "ru", label: t("settings:languageRu") },
        { value: "en", label: t("settings:languageEn") },
      ],
      itemToString: (item) => item.label,
      itemToValue: (item) => item.value,
    });
  });

  const testIdPrefix = () => props.testIdPrefix ?? "settings.language";

  return (
    <div class={rootClass}>
      <Select.Root<LangItem>
        collection={collection()}
        value={[language()]}
        positioning={{ placement: "bottom-start", gutter: 4, sameWidth: true }}
        onValueChange={(detail) => {
          const next = detail.value[0];
          if (next && isAppLanguage(next)) {
            void setLanguage(next);
          }
        }}
      >
        <Select.Control class={controlClass}>
          <Select.Trigger
            class={triggerClass}
            aria-label={t("settings:languageSwitcherAria")}
            data-testid={`${testIdPrefix()}.trigger`}
          >
            <Select.ValueText />
            <Select.Indicator class={indicatorClass}>
              <ChevronDown size={16} strokeWidth={2} aria-hidden="true" />
            </Select.Indicator>
          </Select.Trigger>
        </Select.Control>

        <Select.Positioner class={positionerClass}>
          <Select.Content class={contentClass}>
            <Select.List class={listClass}>
              <For each={collection().items}>
                {(item) => (
                  <Select.Item class={itemClass} item={item} data-testid={`${testIdPrefix()}.${item.value}`}>
                    <Select.ItemText>{item.label}</Select.ItemText>
                    <Select.ItemIndicator class={itemIndicatorClass}>✓</Select.ItemIndicator>
                  </Select.Item>
                )}
              </For>
            </Select.List>
          </Select.Content>
        </Select.Positioner>
      </Select.Root>
    </div>
  );
}
