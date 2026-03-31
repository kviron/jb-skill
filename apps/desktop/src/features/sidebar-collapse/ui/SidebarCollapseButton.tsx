import { PanelLeftClose, PanelLeftOpen } from "lucide-solid";
import { Show } from "solid-js";

import { toolbarButton } from "../../../styled-system/recipes";

type Props = {
  collapsed: boolean;
  onToggle: () => void;
  testId?: string;
};

export function SidebarCollapseButton(props: Props) {
  return (
    <button
      class={toolbarButton()}
      data-testid={props.testId ?? "sidebar.collapse"}
      aria-label={props.collapsed ? "Развернуть меню" : "Свернуть меню"}
      onClick={props.onToggle}
    >
      <Show when={props.collapsed} fallback={<PanelLeftClose size={16} strokeWidth={1.8} />}>
        <PanelLeftOpen size={16} strokeWidth={1.8} />
      </Show>
    </button>
  );
}
