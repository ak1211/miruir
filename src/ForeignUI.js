"use strict";

var R = require('react');
var RN = require('react-native');

//
exports.renderListItem = ({item}) => {
  return R.createElement (
    RN.View,
    { key: item.key, margin: 3, padding: 3 },
    [
      R.createElement (
        RN.Text,
        { key: "A" + item.key },
        item.millisON + "on", ", ", item.millisOFF + "off"
      )
    ]
  );
};
