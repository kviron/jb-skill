/* eslint-disable */
import type { ConditionalValue } from '../types/index';
import type { DistributiveOmit, Pretty } from '../types/system-types';

interface PanelHeaderVariant {
  
}

type PanelHeaderVariantMap = {
  [key in keyof PanelHeaderVariant]: Array<PanelHeaderVariant[key]>
}



export type PanelHeaderVariantProps = {
  [key in keyof PanelHeaderVariant]?: ConditionalValue<PanelHeaderVariant[key]> | undefined
}

export interface PanelHeaderRecipe {
  
  __type: PanelHeaderVariantProps
  (props?: PanelHeaderVariantProps): string
  raw: (props?: PanelHeaderVariantProps) => PanelHeaderVariantProps
  variantMap: PanelHeaderVariantMap
  variantKeys: Array<keyof PanelHeaderVariant>
  splitVariantProps<Props extends PanelHeaderVariantProps>(props: Props): [PanelHeaderVariantProps, Pretty<DistributiveOmit<Props, keyof PanelHeaderVariantProps>>]
  getVariantProps: (props?: PanelHeaderVariantProps) => PanelHeaderVariantProps
}


export declare const panelHeader: PanelHeaderRecipe