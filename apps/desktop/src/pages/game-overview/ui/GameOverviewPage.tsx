import { css } from "../../../styled-system/css";
import { panelHeader } from "../../../styled-system/recipes";
import { UiBadge, UiPanel } from "../../../shared/ui/primitives";

type Props = {
  activeProfileId: string;
  modsCount: number;
  conflictsCount: number;
};

export function GameOverviewPage(props: Props) {
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
      <h2 class={titleClass}>Game Overview</h2>
      <p class={subtitleClass}>Обзор состояния активного профиля и коллекции модов</p>
      <div class={gridClass}>
        <UiPanel>
          <div class={panelHeader()}>Runtime</div>
          <div class={rowClass}><span>Игра</span><UiBadge tone="info">pilot-game</UiBadge></div>
        </UiPanel>
        <UiPanel>
          <div class={panelHeader()}>Profile</div>
          <div class={rowClass}><span>Профиль</span><UiBadge tone="success">{props.activeProfileId}</UiBadge></div>
        </UiPanel>
        <UiPanel>
          <div class={panelHeader()}>Inventory</div>
          <div class={rowClass}><span>Модов</span><UiBadge tone="warning">{props.modsCount}</UiBadge></div>
        </UiPanel>
        <UiPanel>
          <div class={panelHeader()}>Diagnostics</div>
          <div class={rowClass}><span>Конфликтов</span><UiBadge tone="error">{props.conflictsCount}</UiBadge></div>
        </UiPanel>
      </div>
    </>
  );
}
