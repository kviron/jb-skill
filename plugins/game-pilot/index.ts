import { detectGame } from "./detectors";
import { parseMod } from "./installers";
import { planDeploy, planInstall } from "./deploy";
import { validate } from "./validators";

export const plugin = {
  detectGame,
  parseMod,
  planInstall,
  planDeploy,
  validate,
};
