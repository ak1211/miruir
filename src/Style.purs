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
module Style where

import React.Basic (JSX)
import React.Basic.DOM (CSS, css)
import React.Basic.Native as RN

titleText :: CSS
titleText = css { marginVertical: 6, fontSize: 28, fontWeight: "500" }

subTitleText :: CSS
subTitleText = css { marginVertical: 2, fontWeight: "700" }

buttonArea :: CSS
buttonArea = css { marginVertical: 6, alignItems: "flex-start" }

ircodeArea :: CSS
ircodeArea = css { marginVertical: 6, minHeight: 200, alignContent: "stretch" }

hline :: JSX
hline =
  RN.view
    { style: css { margin: 0, height: 1, borderStyle: "solid", borderWidth: 1.0, borderColor: "#999", width: "100%" }
    , children: [ RN.text_ [ RN.string "-------------------" ] ]
    }
