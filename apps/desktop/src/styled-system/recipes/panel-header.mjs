import { memo, splitProps } from '../helpers.mjs';
import { createRecipe, mergeRecipes } from './create-recipe.mjs';

const panelHeaderFn = /* @__PURE__ */ createRecipe('ui-panel-header', {}, [])

const panelHeaderVariantMap = {}

const panelHeaderVariantKeys = Object.keys(panelHeaderVariantMap)

export const panelHeader = /* @__PURE__ */ Object.assign(memo(panelHeaderFn.recipeFn), {
  __recipe__: true,
  __name__: 'panelHeader',
  __getCompoundVariantCss__: panelHeaderFn.__getCompoundVariantCss__,
  raw: (props) => props,
  variantKeys: panelHeaderVariantKeys,
  variantMap: panelHeaderVariantMap,
  merge(recipe) {
    return mergeRecipes(this, recipe)
  },
  splitVariantProps(props) {
    return splitProps(props, panelHeaderVariantKeys)
  },
  getVariantProps: panelHeaderFn.getVariantProps,
})