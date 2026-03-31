import type { Component } from "solid-js";
import {
  AlertTriangle,
  Boxes,
  ClipboardList,
  FolderKanban,
  Gamepad2,
  Settings,
} from "lucide-solid";

export type NavIconName =
  | "overview"
  | "profiles"
  | "mods"
  | "conflicts"
  | "operations"
  | "settings";

export const navIcons: Record<NavIconName, Component<{ size?: number; strokeWidth?: number }>> = {
  overview: Gamepad2,
  profiles: FolderKanban,
  mods: Boxes,
  conflicts: AlertTriangle,
  operations: ClipboardList,
  settings: Settings,
};
