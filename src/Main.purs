module Main (app) where

import Data.Array as Array
import Data.Maybe (Maybe(..), fromMaybe)
import InfraredCodeView (view) as IR
import Prelude
import React.Basic (Component, JSX, createComponent, make)
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
mainView :: Props -> JSX
mainView =
  make component
    { initialState:
        { text: Nothing
        }
    , render:
        \self ->
          React.fragment
            $ Array.concat
                [ [ RN.text
                      { style: titleText
                      , children: [ RN.string ("Edit codes") ]
                      }
                  , RN.view
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
                , case self.state.text of
                    Just text -> [ IR.view text ]
                    Nothing -> []
                ]
    }
  where
  onPressReset self = do
    let
      eventFn1 = RNE.nativeEvent
    RNE.capture eventFn1 \e -> self.setState \s -> s { text = Nothing }

  onChangeTextInput self = do
    let
      (eventFn1 :: EventFn (RNE.NativeSyntheticEvent RN.TextInputChangeEventData) RN.TextInputChangeEventData) = RNE.nativeEvent
    RNE.capture eventFn1 \nativeEvent -> self.setState _ { text = Just nativeEvent.text }
