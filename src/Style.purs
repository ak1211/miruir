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
