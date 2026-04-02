import { type Component } from "solid-js";
import { useTranslation } from "solid-i18next";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { isTauri } from "@tauri-apps/api/core";
import { Minus, Square, X } from "lucide-solid";
import { css, cx } from "../../../styled-system/css";

const controlsClass = css({
  display: "inline-flex",
  flexShrink: "0",
  alignItems: "stretch",
  ml: "auto",
});

const controlBtnClass = css({
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  w: "46px",
  minW: "46px",
  minH: "32px",
  p: "0",
  borderWidth: "0",
  borderStyle: "none",
  bg: "transparent",
  color: "text.secondary",
  cursor: "pointer",
  transition: "background-color 120ms ease",
  _hover: {
    bg: "bg.hover",
    color: "text.primary",
  },
  _focusVisible: {
    outline: "2px solid",
    outlineColor: "focus.ring",
    outlineOffset: "-2px",
  },
});

const closeBtnClass = css({
  _hover: {
    bg: "state.error.bg",
    color: "state.error.text",
  },
});

export const WindowFrameControls: Component = () => {
  const [t] = useTranslation("common");

  const win = () => getCurrentWindow();

  const run = (fn: () => Promise<void>) => {
    if (!isTauri()) return;
    void fn().catch(() => undefined);
  };

  return (
    <div class={controlsClass} role="group" aria-label={t("windowFrameControlsAria")}>
      <button
        type="button"
        class={controlBtnClass}
        aria-label={t("windowMinimize")}
        onClick={() => run(() => win().minimize())}
      >
        <Minus size={16} strokeWidth={2} aria-hidden="true" />
      </button>
      <button
        type="button"
        class={controlBtnClass}
        aria-label={t("windowMaximize")}
        onClick={() => run(() => win().toggleMaximize())}
      >
        <Square size={14} strokeWidth={2} aria-hidden="true" />
      </button>
      <button
        type="button"
        class={cx(controlBtnClass, closeBtnClass)}
        aria-label={t("windowClose")}
        onClick={() => run(() => win().close())}
      >
        <X size={16} strokeWidth={2} aria-hidden="true" />
      </button>
    </div>
  );
};
