import { css } from "../../../styled-system/css";
import { UiInput } from "../primitives";

type Props = {
  value: string;
  onInput: (value: string) => void;
  placeholder: string;
  ariaLabel: string;
};

const wrapClass = css({
  display: "grid",
  gap: "2",
  maxW: "680px",
});

const inputClass = css({
  bg: "bg.elevated",
  borderColor: "border.strong",
});

export function SettingsSearchField(props: Props) {
  return (
    <div class={wrapClass}>
      <UiInput
        class={inputClass}
        value={props.value}
        onInput={(event) => props.onInput(event.currentTarget.value)}
        placeholder={props.placeholder}
        aria-label={props.ariaLabel}
      />
    </div>
  );
}
