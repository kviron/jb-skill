import { css } from "../../../styled-system/css";
import { useTranslation } from "solid-i18next";
import { panelHeader } from "../../../styled-system/recipes";
import { UiBadge, UiPanel } from "../../../shared/ui/primitives";

type Props = {
  activeProfileId: string;
  modsCount: number;
  conflictsCount: number;
};

export function GameOverviewPage(props: Props) {
  const [t] = useTranslation(["common", "pages"]);
  const titleClass = css({ m: "0 0 2", fontSize: "lg", fontWeight: "semibold" });
  const subtitleClass = css({ m: "0 0 4", color: "text.secondary", fontSize: "sm" });
  const gridClass = css({
    display: "grid",
    gridTemplateColumns: "repeat(auto-fit, minmax(220px, 1fr))",
    gap: "3",
  });
  const rowClass = css({ display: "flex", justifyContent: "space-between", alignItems: "center", gap: "2" });

  return (
    <>
      <h2 class={titleClass}>{t("pages:overviewTitle")}</h2>
      <p class={subtitleClass}>{t("pages:overviewSubtitle")}</p>
      <div class={gridClass}>
        <UiPanel>
          <div class={panelHeader()}>{t("pages:runtime")}</div>
          <div class={rowClass}><span>{t("common:gameLabel")}</span><UiBadge tone="info">pilot-game</UiBadge></div>
        </UiPanel>
        <UiPanel>
          <div class={panelHeader()}>{t("pages:profile")}</div>
          <div class={rowClass}><span>{t("common:profileLabel")}</span><UiBadge tone="success">{props.activeProfileId}</UiBadge></div>
        </UiPanel>
        <UiPanel>
          <div class={panelHeader()}>{t("pages:inventory")}</div>
          <div class={rowClass}><span>{t("common:modsLabel")}</span><UiBadge tone="warning">{props.modsCount}</UiBadge></div>
        </UiPanel>
        <UiPanel>
          <div class={panelHeader()}>{t("pages:diagnostics")}</div>
          <div class={rowClass}><span>{t("common:conflictsLabel")}</span><UiBadge tone="error">{props.conflictsCount}</UiBadge></div>
        </UiPanel>
      </div>
    </>
  );
}
