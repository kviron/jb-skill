type Props = {
  activeProfileId: string;
  switchingProfile: boolean;
  onSwitchProfile: () => void;
};

export function ProfilesPage(props: Props) {
  return (
    <>
      <h2>Profiles</h2>
      <p>Текущий профиль: {props.activeProfileId}</p>
      <button data-testid="profiles.switch-button" onClick={props.onSwitchProfile} disabled={props.switchingProfile}>
        {props.switchingProfile ? "Переключение..." : "Переключить профиль"}
      </button>
    </>
  );
}
