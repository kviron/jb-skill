import { SunMoon } from "lucide-solid";
import { useTranslation } from "solid-i18next";

import { useTheme } from "../../../app/providers/theme";
import { toolbarButton } from "../../../styled-system/recipes";

type Props = {
  testId?: string;
  ariaLabel?: string;
};

export function ThemeSwitchButton(props: Props) {
  const [t] = useTranslation("common");
  const { setTheme } = useTheme();

  return (
    <button
      class={toolbarButton()}
      data-testid={props.testId ?? "theme.switch"}
      aria-label={props.ariaLabel ?? t("themeToggleAria")}
      onClick={() => setTheme((current) => (current === "dark" ? "light" : "dark"))}
    >
      <SunMoon size={16} strokeWidth={1.8} />
    </button>
  );
}
