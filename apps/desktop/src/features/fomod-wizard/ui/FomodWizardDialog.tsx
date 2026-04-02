import { For, Match, Show, Switch, createEffect, createSignal } from "solid-js";
import { useTranslation } from "solid-i18next";

import type {
  FomodSelections,
  FomodWizardGroup,
  FomodWizardPayload,
  FomodWizardStep,
} from "../../../shared/api/core";
import { css } from "../../../styled-system/css";
import { UiButton } from "../../../shared/ui/primitives";

function buildDefaultSelections(wizard: FomodWizardPayload): FomodSelections {
  return {
    steps: wizard.steps.map((step, stepIndex) => ({
      stepIndex,
      groups: step.groups.map((group) => {
        const n = group.plugins.length;
        let pluginIndices: number[] = [];
        switch (group.groupType) {
          case "All":
            pluginIndices = group.plugins.map((p) => p.pluginIndex);
            break;
          default:
            pluginIndices = n > 0 ? [group.plugins[0].pluginIndex] : [];
        }
        return { groupIndex: group.groupIndex, pluginIndices };
      }),
    })),
  };
}

function validateSelections(wizard: FomodWizardPayload, sel: FomodSelections): boolean {
  for (let si = 0; si < wizard.steps.length; si++) {
    const step = wizard.steps[si];
    const stepSel = sel.steps.find((s) => s.stepIndex === si);
    if (!stepSel) {
      return false;
    }
    for (let gi = 0; gi < step.groups.length; gi++) {
      const g = step.groups[gi];
      if (g.plugins.length === 0) {
        continue;
      }
      const gSel = stepSel.groups.find((x) => x.groupIndex === gi);
      if (!gSel) {
        return false;
      }
      const n = gSel.pluginIndices.length;
      const t = g.groupType;
      const validIdx = new Set(g.plugins.map((p) => p.pluginIndex));
      for (const idx of gSel.pluginIndices) {
        if (!validIdx.has(idx)) {
          return false;
        }
      }
      if (t === "SelectExactlyOne" && n !== 1) {
        return false;
      }
      if ((t === "SelectAny" || t === "SelectAtLeastOne") && n < 1) {
        return false;
      }
      if (t === "All" && n !== g.plugins.length) {
        return false;
      }
      if (t === "SelectAll" && n < 1) {
        return false;
      }
    }
  }
  return true;
}

const overlayClass = css({
  position: "fixed",
  inset: "0",
  zIndex: 100,
  bg: "rgba(0,0,0,0.5)",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  p: "4",
});

const panelClass = css({
  bg: "bg.surface",
  color: "text.primary",
  borderWidth: "1px",
  borderStyle: "solid",
  borderColor: "border.strong",
  borderRadius: "md",
  maxW: "640px",
  w: "100%",
  maxH: "min(85vh, 720px)",
  overflowY: "auto",
  p: "4",
  display: "flex",
  flexDirection: "column",
  gap: "4",
  boxShadow: "lg",
});

const stepBlockClass = css({
  display: "flex",
  flexDirection: "column",
  gap: "3",
});

const groupBlockClass = css({
  display: "flex",
  flexDirection: "column",
  gap: "2",
  pl: "2",
  borderLeftWidth: "2px",
  borderLeftStyle: "solid",
  borderLeftColor: "border.default",
});

const labelRowClass = css({ display: "flex", flexDirection: "column", gap: "1" });
const optionRowClass = css({ display: "flex", alignItems: "flex-start", gap: "2" });

type Props = {
  wizard: FomodWizardPayload;
  onConfirm: (selections: FomodSelections) => void;
  onCancel: () => void;
};

function updateGroupSelection(
  prev: FomodSelections,
  stepIndex: number,
  groupIndex: number,
  pluginIndices: number[],
): FomodSelections {
  return {
    steps: prev.steps.map((s) => {
      if (s.stepIndex !== stepIndex) {
        return s;
      }
      return {
        ...s,
        groups: s.groups.map((g) => (g.groupIndex === groupIndex ? { ...g, pluginIndices } : g)),
      };
    }),
  };
}

function GroupField(props: {
  step: FomodWizardStep;
  group: FomodWizardGroup;
  selection: FomodSelections;
  onChange: (next: FomodSelections) => void;
}) {
  const stepIndex = () => props.step.stepIndex;
  const groupIndex = () => props.group.groupIndex;
  const selected = () => {
    const stepSel = props.selection.steps.find((s) => s.stepIndex === stepIndex());
    const gSel = stepSel?.groups.find((g) => g.groupIndex === groupIndex());
    return new Set(gSel?.pluginIndices ?? []);
  };

  const setIndices = (indices: number[]) => {
    props.onChange(updateGroupSelection(props.selection, stepIndex(), groupIndex(), indices));
  };

  const toggleIndex = (pluginIndex: number) => {
    const s = new Set(selected());
    if (s.has(pluginIndex)) {
      s.delete(pluginIndex);
    } else {
      s.add(pluginIndex);
    }
    setIndices(Array.from(s).sort((a, b) => a - b));
  };

  return (
    <Switch>
      <Match when={props.group.groupType === "All"}>
        <p class={css({ fontSize: "sm", color: "text.secondary", m: "0" })}>
          {props.group.plugins.length} components
        </p>
      </Match>
      <Match when={props.group.groupType === "SelectExactlyOne"}>
        <For each={props.group.plugins}>
          {(plugin) => (
            <label class={optionRowClass}>
              <input
                type="radio"
                name={`fomod-s${props.step.stepIndex}-g${props.group.groupIndex}`}
                checked={selected().has(plugin.pluginIndex)}
                onChange={() => setIndices([plugin.pluginIndex])}
              />
              <span class={labelRowClass}>
                <span>{plugin.name}</span>
                <Show when={plugin.description}>
                  <span class={css({ fontSize: "xs", color: "text.secondary" })}>{plugin.description}</span>
                </Show>
              </span>
            </label>
          )}
        </For>
      </Match>
      <Match when={true}>
        <For each={props.group.plugins}>
          {(plugin) => (
            <label class={optionRowClass}>
              <input
                type="checkbox"
                checked={selected().has(plugin.pluginIndex)}
                onChange={() => toggleIndex(plugin.pluginIndex)}
              />
              <span class={labelRowClass}>
                <span>{plugin.name}</span>
                <Show when={plugin.description}>
                  <span class={css({ fontSize: "xs", color: "text.secondary" })}>{plugin.description}</span>
                </Show>
              </span>
            </label>
          )}
        </For>
      </Match>
    </Switch>
  );
}

export function FomodWizardDialog(props: Props) {
  const [t] = useTranslation(["common", "pages"]);
  const [selections, setSelections] = createSignal<FomodSelections>(buildDefaultSelections(props.wizard));
  const [localError, setLocalError] = createSignal<string | null>(null);

  createEffect(() => {
    setSelections(buildDefaultSelections(props.wizard));
    setLocalError(null);
  });

  function submit() {
    if (!validateSelections(props.wizard, selections())) {
      setLocalError(t("pages:fomodValidationError"));
      return;
    }
    setLocalError(null);
    props.onConfirm(selections());
  }

  return (
    <div class={overlayClass} role="presentation" onClick={() => props.onCancel()}>
      <div
        class={panelClass}
        role="dialog"
        aria-modal="true"
        aria-labelledby="fomod-wizard-title"
        onClick={(e) => e.stopPropagation()}
      >
        <h2 id="fomod-wizard-title" class={css({ m: "0", fontSize: "lg", fontWeight: "semibold" })}>
          {props.wizard.moduleName}
        </h2>
        <p class={css({ m: "0", fontSize: "sm", color: "text.secondary" })}>{t("pages:fomodWizardSubtitle")}</p>

        <For each={props.wizard.steps}>
          {(step) => (
            <div class={stepBlockClass}>
              <h3 class={css({ m: "0", fontSize: "md", fontWeight: "medium" })}>{step.name}</h3>
              <For each={step.groups}>
                {(group) => (
                  <div class={groupBlockClass}>
                    <span class={css({ fontSize: "sm", fontWeight: "medium" })}>{group.name}</span>
                    <span class={css({ fontSize: "xs", color: "text.secondary" })}>{group.groupType}</span>
                    <GroupField
                      step={step}
                      group={group}
                      selection={selections()}
                      onChange={(next) => setSelections(next)}
                    />
                  </div>
                )}
              </For>
            </div>
          )}
        </For>

        <Show when={localError()}>
          <p role="alert" class={css({ color: "red.11", fontSize: "sm", m: "0" })}>
            {localError()}
          </p>
        </Show>

        <div class={css({ display: "flex", gap: "2", justifyContent: "flex-end", mt: "2" })}>
          <UiButton type="button" variant="outline" onClick={() => props.onCancel()}>
            {t("common:cancel")}
          </UiButton>
          <UiButton type="button" variant="solid" onClick={() => submit()}>
            {t("pages:fomodConfirmInstall")}
          </UiButton>
        </div>
      </div>
    </div>
  );
}
