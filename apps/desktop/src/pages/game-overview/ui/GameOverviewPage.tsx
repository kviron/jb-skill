type Props = {
  activeProfileId: string;
  modsCount: number;
  conflictsCount: number;
};

export function GameOverviewPage(props: Props) {
  return (
    <>
      <h2>Game Overview</h2>
      <p>Игра: pilot-game</p>
      <p>Профиль: {props.activeProfileId}</p>
      <p>Модов: {props.modsCount}</p>
      <p>Конфликтов: {props.conflictsCount}</p>
    </>
  );
}
