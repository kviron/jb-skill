/* eslint-disable */
import type { ConditionalValue } from '../types/index';
import type { DistributiveOmit, Pretty } from '../types/system-types';

interface ActivityItemVariant {
  active: boolean
}

type ActivityItemVariantMap = {
  [key in keyof ActivityItemVariant]: Array<ActivityItemVariant[key]>
}



export type ActivityItemVariantProps = {
  [key in keyof ActivityItemVariant]?: ConditionalValue<ActivityItemVariant[key]> | undefined
}

export interface ActivityItemRecipe {
  
  __type: ActivityItemVariantProps
  (props?: ActivityItemVariantProps): string
  raw: (props?: ActivityItemVariantProps) => ActivityItemVariantProps
  variantMap: ActivityItemVariantMap
  variantKeys: Array<keyof ActivityItemVariant>
  splitVariantProps<Props extends ActivityItemVariantProps>(props: Props): [ActivityItemVariantProps, Pretty<DistributiveOmit<Props, keyof ActivityItemVariantProps>>]
  getVariantProps: (props?: ActivityItemVariantProps) => ActivityItemVariantProps
}


export declare const activityItem: ActivityItemRecipe