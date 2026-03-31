/* eslint-disable */
import type { ConditionalValue } from '../types/index';
import type { DistributiveOmit, Pretty } from '../types/system-types';

interface StatusBarVariant {
  
}

type StatusBarVariantMap = {
  [key in keyof StatusBarVariant]: Array<StatusBarVariant[key]>
}



export type StatusBarVariantProps = {
  [key in keyof StatusBarVariant]?: ConditionalValue<StatusBarVariant[key]> | undefined
}

export interface StatusBarRecipe {
  
  __type: StatusBarVariantProps
  (props?: StatusBarVariantProps): string
  raw: (props?: StatusBarVariantProps) => StatusBarVariantProps
  variantMap: StatusBarVariantMap
  variantKeys: Array<keyof StatusBarVariant>
  splitVariantProps<Props extends StatusBarVariantProps>(props: Props): [StatusBarVariantProps, Pretty<DistributiveOmit<Props, keyof StatusBarVariantProps>>]
  getVariantProps: (props?: StatusBarVariantProps) => StatusBarVariantProps
}


export declare const statusBar: StatusBarRecipe