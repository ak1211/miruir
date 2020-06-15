{-
 miruir <https://github.com/ak1211/miruir>
 Copyright 2019 Akihiro Yamamoto

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
-}
module Main (app) where

import Data.Array ((:))
import Data.Array as Array
import Data.Maybe (Maybe(..), fromMaybe)
import InfraredCodeView (view) as IR
import Prelude
import React.Basic (Component, JSX, Self, createComponent, make)
import React.Basic (fragment) as React
import React.Basic.Events (EventFn)
import React.Basic.Native (TextInputChangeEventData, button, scrollView_, string, text, textInput, view) as RN
import React.Basic.Native.Events (NativeSyntheticEvent, capture, nativeEvent) as RNE
import Style (buttonArea, ircodeArea, titleText)

-- |
app :: JSX
app = RN.scrollView_ [ mainView unit ]

-- |
component :: Component Props
component = createComponent "MainView"

-- |
type Props
  = Unit

-- |
type State
  = { text :: Maybe String }

-- |
mainView :: Props -> JSX
mainView =
  make component
    { initialState:
        { text: Nothing
        }
    , render:
        \self ->
          let
            partial = heading : contents self
          in
            case self.state.text of
              Nothing -> React.fragment partial
              Just text -> React.fragment (partial `Array.snoc` IR.view text)
    }
  where
  heading :: JSX
  heading =
    RN.text
      { style: titleText
      , children: [ RN.string ("Edit codes") ]
      }

  contents :: Self Props State -> Array JSX
  contents self =
    [ RN.view
        { style: buttonArea
        , children: [ RN.button { onPress: onPressReset self, title: "Reset" } ]
        }
    , RN.textInput
        { style: ircodeArea
        , multiline: true
        , numberOfLines: 5.0
        , placeholder: "Write an on-off pair count (32-bit little endianness) hexadecimal number or json made with 'pigpio irrp.py' file."
        , onChange: onChangeTextInput self
        , value: fromMaybe "" self.state.text
        }
    ]

  onPressReset self = do
    let
      eventFn1 = RNE.nativeEvent
    RNE.capture eventFn1 \e -> self.setState \s -> s { text = Nothing }

  onChangeTextInput self = do
    let
      (eventFn1 :: EventFn (RNE.NativeSyntheticEvent RN.TextInputChangeEventData) RN.TextInputChangeEventData) = RNE.nativeEvent
    RNE.capture eventFn1 \nativeEvent -> self.setState _ { text = Just nativeEvent.text }
