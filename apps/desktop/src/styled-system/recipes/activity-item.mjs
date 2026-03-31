import { memo, splitProps } from '../helpers.mjs';
import { createRecipe, mergeRecipes } from './create-recipe.mjs';

const activityItemFn = /* @__PURE__ */ createRecipe('ui-activity-item', {}, [])

const activityItemVariantMap = {
  "active": [
    "true"
  ]
}

const activityItemVariantKeys = Object.keys(activityItemVariantMap)

export const activityItem = /* @__PURE__ */ Object.assign(memo(activityItemFn.recipeFn), {
  __recipe__: true,
  __name__: 'activityItem',
  __getCompoundVariantCss__: activityItemFn.__getCompoundVariantCss__,
  raw: (props) => props,
  variantKeys: activityItemVariantKeys,
  variantMap: activityItemVariantMap,
  merge(recipe) {
    return mergeRecipes(this, recipe)
  },
  splitVariantProps(props) {
    return splitProps(props, activityItemVariantKeys)
  },
  getVariantProps: activityItemFn.getVariantProps,
})