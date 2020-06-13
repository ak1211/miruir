module ForeignUI
  ( RenderListItem
  , renderListItem
  ) where

import Effect.Uncurried (EffectFn1)
import React.Basic (JSX)
import React.Basic.Native (ListRenderItemInfo)

-- |
type RenderListItem
  = { key :: String, millisON :: String, millisOFF :: String }

-- |
foreign import renderListItem :: EffectFn1 (ListRenderItemInfo RenderListItem) JSX
