import { memo, splitProps } from '../helpers.mjs';
import { createRecipe, mergeRecipes } from './create-recipe.mjs';

const toolbarButtonFn = /* @__PURE__ */ createRecipe('ui-toolbar-button', {}, [])

const toolbarButtonVariantMap = {}

const toolbarButtonVariantKeys = Object.keys(toolbarButtonVariantMap)

export const toolbarButton = /* @__PURE__ */ Object.assign(memo(toolbarButtonFn.recipeFn), {
  __recipe__: true,
  __name__: 'toolbarButton',
  __getCompoundVariantCss__: toolbarButtonFn.__getCompoundVariantCss__,
  raw: (props) => props,
  variantKeys: toolbarButtonVariantKeys,
  variantMap: toolbarButtonVariantMap,
  merge(recipe) {
    return mergeRecipes(this, recipe)
  },
  splitVariantProps(props) {
    return splitProps(props, toolbarButtonVariantKeys)
  },
  getVariantProps: toolbarButtonFn.getVariantProps,
})