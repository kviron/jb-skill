import { For } from "solid-js";

import type { ModRecord } from "../../../shared/api/core";

type Props = {
  modArchivePath: string;
  modSearch: string;
  mods: ModRecord[];
  installing: boolean;
  togglingModId: string | null;
  removingModId: string | null;
  onArchivePathInput: (value: string) => void;
  onSearchInput: (value: string) => void;
  onInstall: () => void;
  onToggle: (item: ModRecord) => void;
  onRemove: (item: ModRecord) => void;
};

export function ModsPage(props: Props) {
  return (
    <>
      <h2>Mods List</h2>
      <form
        class="toolbar"
        onSubmit={(event) => {
          event.preventDefault();
          props.onInstall();
        }}
      >
        <label for="archivePath">Архив мода</label>
        <input
          id="archivePath"
          data-testid="mods.archive-input"
          value={props.modArchivePath}
          onInput={(event) => props.onArchivePathInput(event.currentTarget.value)}
          placeholder="C:/mods/my-mod.zip"
        />
        <button type="submit" data-testid="mods.install-button" disabled={props.installing}>
          {props.installing ? "Установка..." : "Установить"}
        </button>
      </form>

      <label for="searchInput">Поиск модов</label>
      <input
        id="searchInput"
        data-testid="mods.search-input"
        value={props.modSearch}
        onInput={(event) => props.onSearchInput(event.currentTarget.value)}
        placeholder="Введите имя мода"
      />

      <ul class="mods-list">
        <For each={props.mods}>
          {(item) => (
            <li>
              <div>
                <strong>{item.name}</strong>
                <span>v{item.version ?? "0.0.0"}</span>
              </div>
              <div class="mod-actions">
                <button
                  data-testid={`mods.toggle.${item.id}`}
                  disabled={props.togglingModId === item.id}
                  onClick={() => props.onToggle(item)}
                >
                  {item.enabled ? "Выключить" : "Включить"}
                </button>
                <button
                  data-testid={`mods.remove.${item.id}`}
                  disabled={props.removingModId === item.id}
                  onClick={() => props.onRemove(item)}
                >
                  Удалить
                </button>
              </div>
            </li>
          )}
        </For>
      </ul>
    </>
  );
}
