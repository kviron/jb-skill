import { type Component } from "solid-js";
import { useTranslation } from "solid-i18next";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { isTauri } from "@tauri-apps/api/core";
import { WindowFrameControls } from "../../../features/window-frame-controls";
import { css } from "../../../styled-system/css";

const barClass = css({
  gridColumn: "1 / -1",
  gridRow: "1",
  display: "flex",
  alignItems: "stretch",
  flexShrink: "0",
  h: "32px",
  minH: "32px",
  bg: "bg.surface",
  borderBottomWidth: "1px",
  borderBottomStyle: "solid",
  borderBottomColor: "border.strong",
  userSelect: "none",
});

const dragClass = css({
  flex: "1",
  display: "flex",
  alignItems: "center",
  gap: "2",
  minW: "0",
  pl: "3",
  pr: "2",
  color: "text.secondary",
  fontSize: "sm",
  fontWeight: "semibold",
});

const brandIconClass = css({
  display: "inline-flex",
  flexShrink: "0",
  color: "icon.default",
});

export const ShellTitleBar: Component = () => {
  const [t] = useTranslation("common");

  const onDblClick = () => {
    if (!isTauri()) return;
    void getCurrentWindow().toggleMaximize().catch(() => undefined);
  };

  return (
    <header class={barClass} data-testid="shell.title-bar">
      <div
        data-tauri-drag-region
        class={dragClass}
        onDblClick={onDblClick}
      >
        <span class={brandIconClass} aria-hidden="true">
          <img src="/pantheon-mark.svg" width={18} height={18} alt="" />
        </span>
        <span
          class={css({
            overflow: "hidden",
            textOverflow: "ellipsis",
            whiteSpace: "nowrap",
          })}
        >
          {t("appName")}
        </span>
      </div>
      <WindowFrameControls />
    </header>
  );
};
