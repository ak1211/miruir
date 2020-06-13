/*
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
*/
"use strict";

var R = require('react');
var RN = require('react-native');

//
exports.renderListItem = ({ item }) => {
  return R.createElement(
    RN.View,
    { key: item.key, margin: 3, padding: 3 },
    [
      R.createElement(
        RN.Text,
        { key: "A" + item.key },
        item.millisON + "on", ", ", item.millisOFF + "off"
      )
    ]
  );
};
