import { memo, splitProps } from '../helpers.mjs';
import { createRecipe, mergeRecipes } from './create-recipe.mjs';

const sideNavItemFn = /* @__PURE__ */ createRecipe('ui-side-nav-item', {}, [])

const sideNavItemVariantMap = {
  "active": [
    "true"
  ],
  "collapsed": [
    "true"
  ]
}

const sideNavItemVariantKeys = Object.keys(sideNavItemVariantMap)

export const sideNavItem = /* @__PURE__ */ Object.assign(memo(sideNavItemFn.recipeFn), {
  __recipe__: true,
  __name__: 'sideNavItem',
  __getCompoundVariantCss__: sideNavItemFn.__getCompoundVariantCss__,
  raw: (props) => props,
  variantKeys: sideNavItemVariantKeys,
  variantMap: sideNavItemVariantMap,
  merge(recipe) {
    return mergeRecipes(this, recipe)
  },
  splitVariantProps(props) {
    return splitProps(props, sideNavItemVariantKeys)
  },
  getVariantProps: sideNavItemFn.getVariantProps,
})