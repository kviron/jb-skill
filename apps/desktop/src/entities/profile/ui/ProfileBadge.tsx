import { UiBadge } from "../../../shared/ui/primitives";

type Props = {
  profileId: string;
};

export function ProfileBadge(props: Props) {
  return <UiBadge tone="info">{props.profileId}</UiBadge>;
}
