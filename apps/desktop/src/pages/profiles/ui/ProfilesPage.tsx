import { css } from "../../../styled-system/css";
import { panelHeader } from "../../../styled-system/recipes";
import { UiButton, UiPanel } from "../../../shared/ui/primitives";
import { ProfileBadge } from "../../../entities";

type Props = {
  activeProfileId: string;
  switchingProfile: boolean;
  onSwitchProfile: () => void;
};

export function ProfilesPage(props: Props) {
  const stackClass = css({ display: "grid", gap: "4", maxW: "560px" });
  const titleClass = css({ m: "0", fontSize: "lg", fontWeight: "semibold" });
  const subtitleClass = css({ m: "0", color: "text.secondary", fontSize: "sm" });
  const rowClass = css({ display: "flex", alignItems: "center", gap: "3", flexWrap: "wrap" });

  return (
    <div class={stackClass}>
      <h2 class={titleClass}>Profiles</h2>
      <p class={subtitleClass}>Управление активным профилем и переключением окружений</p>
      <UiPanel>
        <div class={panelHeader()}>Active Profile</div>
        <div class={rowClass}>
          <span>Текущий профиль:</span>
          <ProfileBadge profileId={props.activeProfileId} />
        </div>
      </UiPanel>
      <UiButton
        variant="solid"
        data-testid="profiles.switch-button"
        onClick={props.onSwitchProfile}
        disabled={props.switchingProfile}
      >
        {props.switchingProfile ? "Переключение..." : "Переключить профиль"}
      </UiButton>
    </div>
  );
}
