/* eslint-disable */
import type { ConditionalValue } from '../types/index';
import type { DistributiveOmit, Pretty } from '../types/system-types';

interface SideNavItemVariant {
  active: boolean
collapsed: boolean
}

type SideNavItemVariantMap = {
  [key in keyof SideNavItemVariant]: Array<SideNavItemVariant[key]>
}



export type SideNavItemVariantProps = {
  [key in keyof SideNavItemVariant]?: ConditionalValue<SideNavItemVariant[key]> | undefined
}

export interface SideNavItemRecipe {
  
  __type: SideNavItemVariantProps
  (props?: SideNavItemVariantProps): string
  raw: (props?: SideNavItemVariantProps) => SideNavItemVariantProps
  variantMap: SideNavItemVariantMap
  variantKeys: Array<keyof SideNavItemVariant>
  splitVariantProps<Props extends SideNavItemVariantProps>(props: Props): [SideNavItemVariantProps, Pretty<DistributiveOmit<Props, keyof SideNavItemVariantProps>>]
  getVariantProps: (props?: SideNavItemVariantProps) => SideNavItemVariantProps
}


export declare const sideNavItem: SideNavItemRecipe