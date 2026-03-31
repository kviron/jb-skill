import { memo, splitProps } from '../helpers.mjs';
import { createRecipe, mergeRecipes } from './create-recipe.mjs';

const statusBarFn = /* @__PURE__ */ createRecipe('ui-status-bar', {}, [])

const statusBarVariantMap = {}

const statusBarVariantKeys = Object.keys(statusBarVariantMap)

export const statusBar = /* @__PURE__ */ Object.assign(memo(statusBarFn.recipeFn), {
  __recipe__: true,
  __name__: 'statusBar',
  __getCompoundVariantCss__: statusBarFn.__getCompoundVariantCss__,
  raw: (props) => props,
  variantKeys: statusBarVariantKeys,
  variantMap: statusBarVariantMap,
  merge(recipe) {
    return mergeRecipes(this, recipe)
  },
  splitVariantProps(props) {
    return splitProps(props, statusBarVariantKeys)
  },
  getVariantProps: statusBarFn.getVariantProps,
})