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
module InfraredCodeView (view) where

import Prelude
import Data.Array ((:))
import Data.Array as Array
import Data.Array.NonEmpty (NonEmptyArray)
import Data.Bifunctor as Bifunctor
import Data.Either (Either(..), either)
import Data.Foldable (intercalate)
import Data.Formatter.Number as FN
import Data.FunctorWithIndex (mapWithIndex)
import Data.Int as Int
import Data.Newtype (unwrap)
import Data.String as String
import Data.Traversable (traverse)
import Data.Tuple (Tuple(..))
import ForeignUI (RenderListItem, renderListItem) as FUI
import InfraredRemoteCode (Baseband(..), Bit, BitOrder, Count, InfraredCodeFrame(..), InfraredLeader(..), IrRemoteControlCode(..), ProcessError, Pulse, decodePhase1, decodePhase2, decodePhase3, decodePhase4, infraredCodeTextParser, toInfraredHexString, toLsbFirst, toMilliseconds, unBitOrder)
import InfraredRemoteCode.Devices.DaikinHvac (DaikinHvac(..))
import InfraredRemoteCode.Devices.DaikinHvac as Da
import InfraredRemoteCode.Devices.HitachiHvac (HitachiHvac(..))
import InfraredRemoteCode.Devices.MitsubishiElectricHvac (MitsubishiElectricHvac(..))
import InfraredRemoteCode.Devices.MitsubishiElectricHvac as Me
import InfraredRemoteCode.Devices.PanasonicHvac (PanasonicHvac(..))
import InfraredRemoteCode.Devices.PanasonicHvac as Pa
import InfraredRemoteCode.Devices.SIRC (SIRC(..))
import React.Basic (Component, JSX, createComponent, makeStateless)
import React.Basic (fragment) as React
import React.Basic.DOM (css)
import React.Basic.Native (flatList, string, text, text_, view, view_) as RN
import Style (hline, subTitleText, titleText)
import Text.Parsing.Parser (parseErrorMessage, runParser)
import Utils (toArrayArray)

-- |
type Props
  = String

-- |
component :: Component Props
component = createComponent "InfraredCodeView"

-- |
view :: Props -> JSX
view =
  makeStateless component \props ->
    let
      rawcode = props

      baseband = Bifunctor.lmap parseErrorMessage (runParser rawcode infraredCodeTextParser)

      bitPatterns = (traverse decodePhase2 <<< decodePhase1) =<< baseband

      irframes = traverse decodePhase3 =<< bitPatterns

      irRemoteCodes = Bifunctor.rmap decodePhase4 irframes
    in
      case baseband of
        Left err -> RN.text { children: [ RN.string err ] }
        Right ok ->
          RN.view_
            [ binariesView ok
            , timingTableView ok
            , bitPatternsView bitPatterns
            , infraredRemoteControlFramesView irframes
            , infraredRemoteControlCodeView irRemoteCodes
            ]

-- |
binariesView :: Baseband -> JSX
binariesView baseband =
  RN.view_
    [ RN.text
        { style: titleText
        , children: [ RN.string ("Binaries") ]
        }
    , RN.text { children: [ RN.string $ toInfraredHexString baseband ] }
    ]

-- |
timingTableView :: Baseband -> JSX
timingTableView bb =
  RN.view
    { style: css {}
    , children: [ heading, content bb ]
    }
  where
  heading :: JSX
  heading =
    RN.view_
      [ RN.text
          { style: titleText
          , children: [ RN.string ("Timing table in milliseconds") ]
          }
      ]

  content :: Baseband -> JSX
  content (Baseband pulses) =
    RN.flatList
      { data: mapWithIndex col pulses
      , renderItem: FUI.renderListItem
      , numColumns: 4.0
      , contentContainerStyle: css {}
      }

  strMillisec :: Count -> String
  strMillisec n =
    either (const "N/A") identity
      $ FN.formatNumber "0.00"
      $ unwrap
      $ toMilliseconds n

  col :: Int -> Pulse -> FUI.RenderListItem
  col index pulse =
    { key: "millisec" <> show index
    , millisON: strMillisec pulse.on
    , millisOFF: strMillisec pulse.off
    }

-- |
bitPatternsView :: Either ProcessError (Array (Tuple InfraredLeader (Array Bit))) -> JSX
bitPatternsView bitpatterns =
  RN.view
    { style: css {}
    , children: heading : either errContents contents bitpatterns
    }
  where
  heading :: JSX
  heading =
    RN.view_
      [ RN.text
          { style: titleText
          , children: [ RN.string "Bit patterns" ]
          }
      ]

  contents :: Array (Tuple InfraredLeader (Array Bit)) -> Array JSX
  contents = intercalate [ hline ] <<< map infraredBitpatterns

  errContents :: String -> Array JSX
  errContents msg = [ RN.view_ [ RN.text_ [ RN.string msg ] ] ]

-- |
infraredBitpatterns :: Tuple InfraredLeader (Array Bit) -> Array JSX
infraredBitpatterns (Tuple leader vs) = case leader of
  LeaderAeha _ ->
    [ RN.text { children: [ RN.string "AEHA" ] }
    , row $ toArrayArray 8 vs
    ]
  LeaderNec _ ->
    [ RN.text { children: [ RN.string "NEC" ] }
    , row $ toArrayArray 8 vs
    ]
  LeaderSirc _ ->
    let
      bit7 = Array.take 7 vs

      left = Array.drop 7 vs
    in
      [ RN.text { children: [ RN.string "SIRC" ] }
      , row (bit7 : toArrayArray 8 left)
      ]
  LeaderUnknown _ ->
    [ RN.text { children: [ RN.string "Unknown" ] }
    , row $ toArrayArray 8 vs
    ]
  where
  row :: Array (Array Bit) -> JSX
  row xxs =
    RN.view
      { style: css { flex: 1, flexDirection: "row", flexWrap: "wrap" }
      , children: map col xxs
      }

  col :: Array Bit -> JSX
  col xs =
    RN.view
      { style: css { margin: 6 }
      , children: [ RN.text_ $ map (RN.string <<< show) xs ]
      }

-- |
infraredRemoteControlFramesView :: Either ProcessError (Array InfraredCodeFrame) -> JSX
infraredRemoteControlFramesView irframes =
  RN.view
    { style: css {}
    , children: heading : either errContents contents irframes
    }
  where
  heading :: JSX
  heading =
    RN.view_
      [ RN.text
          { style: titleText
          , children: [ RN.string "Infrared remote control frames" ]
          }
      ]

  contents :: Array InfraredCodeFrame -> Array JSX
  contents = intercalate [ hline ] <<< map infraredCodeFrame

  errContents :: String -> Array JSX
  errContents msg = [ RN.view_ [ RN.text_ [ RN.string msg ] ] ]

-- |
infraredRemoteControlCodeView :: Either ProcessError (NonEmptyArray IrRemoteControlCode) -> JSX
infraredRemoteControlCodeView irRemoteCodes =
  RN.view
    { style: css {}
    , children: heading : either errContents contents irRemoteCodes
    }
  where
  heading :: JSX
  heading =
    RN.view_
      [ RN.text
          { style: titleText
          , children: [ RN.string "Infrared remote control code" ]
          }
      ]

  contents :: NonEmptyArray IrRemoteControlCode -> Array JSX
  contents = intercalate [ hline ] <<< map infraredRemoteControlCode

  errContents :: String -> Array JSX
  errContents msg = [ RN.view_ [ RN.text_ [ RN.string msg ] ] ]

-- |
infraredCodeFrame :: InfraredCodeFrame -> Array JSX
infraredCodeFrame = case _ of
  FormatNEC irValue ->
    [ RN.text
        { style: subTitleText
        , children: [ RN.string "format" ]
        }
    , RN.text_ [ RN.string "NEC" ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "custom code (LSBit first)" ]
        }
    , RN.text_ $ map (cell <<< formatWithHex <<< toLsbFirst) [ irValue.custom0, irValue.custom1 ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "octets (LSBit first)" ]
        }
    , RN.text_ $ map (cell <<< formatWithHex <<< toLsbFirst) [ irValue.data0, irValue.data1 ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "stop" ]
        }
    , RN.text_ [ RN.string $ show irValue.stop ]
    ]
  FormatAEHA irValue ->
    [ RN.text
        { style: subTitleText
        , children: [ RN.string "format" ]
        }
    , RN.text_ [ RN.string "AEHA" ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "octets(LSBit first)" ]
        }
    , RN.text_ $ map (cell <<< formatWithHex <<< toLsbFirst) irValue.octets
    , RN.text
        { style: subTitleText
        , children: [ RN.string "stop" ]
        }
    , RN.text_ [ RN.string $ show irValue.stop ]
    ]
  FormatSIRC12 irValue ->
    [ RN.text
        { style: subTitleText
        , children: [ RN.string "format" ]
        }
    , RN.text_ [ RN.string "SIRC12" ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "command (LSBit first)" ]
        }
    , RN.text_ [ RN.string $ formatWithHex $ toLsbFirst irValue.command ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "address (LSBit first)" ]
        }
    , RN.text_ [ RN.string $ formatWithHex $ toLsbFirst irValue.address ]
    ]
  FormatSIRC15 irValue ->
    [ RN.text
        { style: subTitleText
        , children: [ RN.string "format" ]
        }
    , RN.text_ [ RN.string "SIRC15" ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "command (LSBit first)" ]
        }
    , RN.text_ [ RN.string $ formatWithHex $ toLsbFirst irValue.command ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "address (LSBit first)" ]
        }
    , RN.text_ [ RN.string $ formatWithHex $ toLsbFirst irValue.address ]
    ]
  FormatSIRC20 irValue ->
    [ RN.text
        { style: subTitleText
        , children: [ RN.string "format" ]
        }
    , RN.text_ [ RN.string "SIRC20" ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "command (LSBit first)" ]
        }
    , RN.text_ [ RN.string $ formatWithHex $ toLsbFirst irValue.command ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "address (LSBit first)" ]
        }
    , RN.text_ [ RN.string $ formatWithHex $ toLsbFirst irValue.address ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "extended (LSBit first)" ]
        }
    , RN.text_ [ RN.string $ formatWithHex $ toLsbFirst irValue.extended ]
    ]
  FormatUnknown irValue ->
    [ RN.text
        { style: subTitleText
        , children: [ RN.string "unknown format" ]
        }
    , RN.text_ [ RN.string $ show irValue ]
    ]
  where
  cell str = React.fragment [ RN.string str, RN.string " " ]

-- |
formatWithHex :: BitOrder -> String
formatWithHex bits =
  let
    str = Int.toStringAs Int.hexadecimal $ unBitOrder bits
  in
    "0x"
      <> case String.length str of
          len
            | len < 2 -> "0" <> str
            | otherwise -> str

-- |
infraredRemoteControlCode :: IrRemoteControlCode -> Array JSX
infraredRemoteControlCode = case _ of
  IrRemoteUnknown formats ->
    [ RN.text
        { style: subTitleText
        , children: [ RN.string "Unknown IR remote Code" ]
        }
    ]
  IrRemoteSIRC (SIRC v) ->
    [ RN.text_ [ RN.string "SIRC" ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Device" ]
        }
    , RN.text_ [ RN.string (show v.device) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Command" ]
        }
    , RN.text_ [ RN.string (show v.command) ]
    ]
  IrRemoteDaikinHvac (DaikinHvac v) ->
    [ RN.text_ [ RN.string "Daikin HVAC" ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Temperature" ]
        }
    , RN.text_ [ RN.string (show v.temperature) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Mode" ]
        }
    , RN.text_ [ RN.string (show v.mode) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Switch" ]
        }
    , RN.text_ [ RN.string (show v.switch) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Fan" ]
        }
    , RN.text_ [ RN.string (show v.fan) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Swing" ]
        }
    , RN.text_ [ RN.string (show v.swing) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "OnTimer" ]
        }
    , RN.text_ [ RN.string (show v.onTimer) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "OnTimerDulationHour" ]
        }
    , RN.text_ [ RN.string (show v.onTimerDulationHour) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "OffTimer" ]
        }
    , RN.text_ [ RN.string (show v.offTimer) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "OffTimerDulationHour" ]
        }
    , RN.text_ [ RN.string (show v.offTimerDulationHour) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Checksum" ]
        }
    , RN.text_
        let
          msg =
            if Da.validChecksum v.checksum v.original then
              "Checksum is valid."
            else
              "Checksum is NOT valid."
        in
          [ RN.string $ msg <> " " <> show v.checksum ]
    ]
  IrRemotePanasonicHvac (PanasonicHvac v) ->
    [ RN.text_ [ RN.string "Panasonic HVAC" ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Temperature" ]
        }
    , RN.text_ [ RN.string (show v.temperature) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Mode" ]
        }
    , RN.text_ [ RN.string (show v.mode) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Switch" ]
        }
    , RN.text_ [ RN.string (show v.switch) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Fan" ]
        }
    , RN.text_ [ RN.string (show v.fan) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Swing" ]
        }
    , RN.text_ [ RN.string (show v.swing) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Profile" ]
        }
    , RN.text_ [ RN.string (show v.profile) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "CRC" ]
        }
    , RN.text_
        let
          msg =
            if Pa.validCrc v.crc v.original then
              "Checksum is valid."
            else
              "Checksum is NOT valid."
        in
          [ RN.string $ msg <> " " <> show v.crc ]
    ]
  IrRemoteMitsubishiElectricHvac (MitsubishiElectricHvac v) ->
    [ RN.text_ [ RN.string "MitsubishiElectric HVAC" ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Temperature" ]
        }
    , RN.text_ [ RN.string (show v.temperature) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Mode1" ]
        }
    , RN.text_ [ RN.string (show v.mode1) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Switch" ]
        }
    , RN.text_ [ RN.string (show v.switch) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "CRC" ]
        }
    , RN.text_
        let
          msg =
            if Me.validCrc v.crc v.original then
              "Checksum is valid."
            else
              "Checksum is NOT valid."
        in
          [ RN.string $ msg <> " " <> show v.crc ]
    ]
  IrRemoteHitachiHvac (HitachiHvac v) ->
    [ RN.text_ [ RN.string "Hitachi HVAC" ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Temperature" ]
        }
    , RN.text_ [ RN.string (show v.temperature) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Mode" ]
        }
    , RN.text_ [ RN.string (show v.mode) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Switch" ]
        }
    , RN.text_ [ RN.string (show v.switch) ]
    , RN.text
        { style: subTitleText
        , children: [ RN.string "Fan" ]
        }
    , RN.text_ [ RN.string (show v.fan) ]
    ]
