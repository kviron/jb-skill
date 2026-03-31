/* eslint-disable */
import type { ConditionalValue } from '../types/index';
import type { DistributiveOmit, Pretty } from '../types/system-types';

interface ToolbarButtonVariant {
  
}

type ToolbarButtonVariantMap = {
  [key in keyof ToolbarButtonVariant]: Array<ToolbarButtonVariant[key]>
}



export type ToolbarButtonVariantProps = {
  [key in keyof ToolbarButtonVariant]?: ConditionalValue<ToolbarButtonVariant[key]> | undefined
}

export interface ToolbarButtonRecipe {
  
  __type: ToolbarButtonVariantProps
  (props?: ToolbarButtonVariantProps): string
  raw: (props?: ToolbarButtonVariantProps) => ToolbarButtonVariantProps
  variantMap: ToolbarButtonVariantMap
  variantKeys: Array<keyof ToolbarButtonVariant>
  splitVariantProps<Props extends ToolbarButtonVariantProps>(props: Props): [ToolbarButtonVariantProps, Pretty<DistributiveOmit<Props, keyof ToolbarButtonVariantProps>>]
  getVariantProps: (props?: ToolbarButtonVariantProps) => ToolbarButtonVariantProps
}


export declare const toolbarButton: ToolbarButtonRecipe