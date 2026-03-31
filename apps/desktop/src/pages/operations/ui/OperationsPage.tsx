import { For } from "solid-js";

type Props = {
  operationLog: string[];
};

export function OperationsPage(props: Props) {
  return (
    <>
      <h2>Operation Log</h2>
      <ul class="logs-list" role="log" aria-label="Журнал операций">
        <For each={props.operationLog}>{(entry) => <li>{entry}</li>}</For>
      </ul>
    </>
  );
}
