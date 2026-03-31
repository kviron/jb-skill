import { css } from "../../../styled-system/css";
import { useTranslation } from "solid-i18next";
import { panelHeader } from "../../../styled-system/recipes";
import { UiButton, UiPanel } from "../../../shared/ui/primitives";
import { ProfileBadge } from "../../../entities";

type Props = {
  activeProfileId: string;
  switchingProfile: boolean;
  onSwitchProfile: () => void;
};

export function ProfilesPage(props: Props) {
  const [t] = useTranslation(["common", "pages"]);
  const stackClass = css({ display: "grid", gap: "4", maxW: "560px" });
  const titleClass = css({ m: "0", fontSize: "lg", fontWeight: "semibold" });
  const subtitleClass = css({ m: "0", color: "text.secondary", fontSize: "sm" });
  const rowClass = css({ display: "flex", alignItems: "center", gap: "3", flexWrap: "wrap" });

  return (
    <div class={stackClass}>
      <h2 class={titleClass}>{t("pages:profilesTitle")}</h2>
      <p class={subtitleClass}>{t("pages:profilesSubtitle")}</p>
      <UiPanel>
        <div class={panelHeader()}>{t("pages:activeProfile")}</div>
        <div class={rowClass}>
          <span>{t("common:profileCurrent")}:</span>
          <ProfileBadge profileId={props.activeProfileId} />
        </div>
      </UiPanel>
      <UiButton
        variant="solid"
        data-testid="profiles.switch-button"
        onClick={props.onSwitchProfile}
        disabled={props.switchingProfile}
      >
        {props.switchingProfile ? t("common:switchingProfile") : t("common:switchProfile")}
      </UiButton>
    </div>
  );
}
