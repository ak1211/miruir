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
import React, { useState } from 'react';
import { StyleSheet, View } from 'react-native';
import { Button, Text } from 'react-native-elements';
import { app } from "./output/Main"

const App = (props) => {
  const [visible, setVisible] = useState(false);
  if (visible) {
    return (
      <View style={styles.about}>
        <View>
          <Text style={{ paddingBottom: 30 }} h2> About this application</Text>
          <Text style={{ paddingBottom: 30 }} h5>miruir &lt;https://github.com/ak1211/miruir&gt;</Text>
          <Text style={{ paddingBottom: 30 }} h5>&copy;2020 Akihiro Yamamoto</Text>
          <Button title='Ok' onPress={() => setVisible(false)} />
        </View>
      </View >
    );
  } else {
    return (
      <>
        <View style={styles.container}>{app}</View>
        <Footer setVisible={setVisible} />
      </>
    );
  }
}

const Footer = (props) => {
  return (
    <>
      <Button
        title='miruir &copy;2020 Akihiro Yamamoto'
        buttonStyle={{ color: '#333333', backgroundColor: '#4f4040' }}
        onPress={() => props.setVisible(true)}
      />
    </>
  );
}

export default App;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f9fdfd',
    alignItems: 'stretch',
    justifyContent: 'flex-start',
    margin: 8,
  },
  about: {
    flex: 1,
    backgroundColor: '#ccc',
    alignItems: 'center',
    justifyContent: 'center',
  },
});
