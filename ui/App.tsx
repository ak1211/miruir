// Copyright (c) 2022 Akihiro Yamamoto.
// Licensed under the MIT License <https://spdx.org/licenses/MIT.html>
// See LICENSE file in the project root for full license information.
//
import { useState } from 'react';
import { Modal, Button, Card, Divider, Input, Alert, Typography, Space, Table, message } from 'antd';
import 'antd/dist/antd.min.css';
import { Line, Datum } from '@ant-design/charts';
import { invoke } from '@tauri-apps/api/tauri'
import { RxIrRemoteCode, TxIrRemoteCode, RxTxIrRemoteCode, MarkAndSpace, convert_to_RxIrRemoteCode, convert_to_TxIrRemoteCode } from './index';
import IrBitStream from './IrBitStream';
import './App.css';

const { TextArea } = Input;
const { Text, Title } = Typography;

//
const InfraredRemoteSignal = (props: { rx_ircode: RxIrRemoteCode }): JSX.Element => {
  let mark_and_spaces: MarkAndSpace[] = props.rx_ircode;

  type DatumForList = { key: any, sn: number, t: number, kinds: string, duration: number }
  const convert_for_list = (input: MarkAndSpace[]): DatumForList[] => {
    var acc: number = 0;
    var output: DatumForList[] = [];
    input.forEach((item, index) => {
      let sequence_number = 1 + 2 * index;
      output.push({ key: sequence_number, sn: sequence_number, t: acc, kinds: "Mark", duration: item.mark });
      acc += item.mark;
      output.push({ key: sequence_number + 1, sn: sequence_number + 1, t: acc, kinds: "Space", duration: item.space });
      acc += item.space;
    })
    return output
  }

  //
  type DatumForGraph = { t: number, bit: number }
  const conv_ir_control_signal = (input: MarkAndSpace[]): Array<DatumForGraph> => {
    if (input.length < 1) {
      return []
    }
    var output: DatumForGraph[] = [];
    var sum = 0;
    output.push({ t: sum, bit: 0 });
    input.forEach((item, index) => {
      sum += item.mark;
      output.push({ t: sum, bit: 1 });
      sum += item.space;
      output.push({ t: sum, bit: 0 });
    })
    return output;
  }

  //
  const config = {
    data: conv_ir_control_signal(mark_and_spaces),
    height: 200,
    xField: 't',
    yField: 'bit',
    stepType: 'vh',
    animation: false,
    xAxis: {
      type: 'time',
      tickInterval: 1,
      label: {
        formatter: (_i: any, _j: any, index: number) => {
          return index
        }
      },
    },
    yAxis: {
      tickInterval: 1,
    },
    tooltip: {
      title: "赤外線リモコン信号",
      formatter: (datum: Datum) => {
        return { name: datum.t + 'μs', value: datum.bit === 0 ? 'Lo' : 'Hi' };
      },
    },
    slider: {
      start: 0.0,
      end: 1.0,
    },
  };

  const columns = [
    {
      title: 'Seqence Number',
      dataIndex: 'sn',
      key: 'sn',
    },
    {
      title: 'Start Time (μs)',
      dataIndex: 't',
      key: 't',
    },
    {
      title: 'Duration (μs)',
      dataIndex: 'duration',
      key: 'duration',
    },
    {
      title: 'Kinds',
      dataIndex: 'kinds',
      key: 'kinds',
    },
  ];

  return (
    <Card size="small" title={<Title level={4}>伝送信号</Title>}>
      <Line {...config} />
      <Divider>マークアンドスペース</Divider>
      <Table
        dataSource={convert_for_list(mark_and_spaces)}
        columns={columns}
        scroll={{ y: 240 }}
      />
    </Card>
  )
}

interface State {
  text: string,
  alert: {
    type: 'success' | 'info' | 'warning' | 'error',
    message: string,
  },
};

const initState: State = {
  text: "",
  alert: {
    type: 'info',
    message: "入力してね。",
  },
};

//
const App = (): JSX.Element => {
  const [state, setState] = useState<State>(initState)
  const [rx_tx_ircode, setRxTxIrCode] = useState<RxTxIrRemoteCode>({ RxIrRemoteCode: [] })

  const handleReset = () => {
    setState(initState)
    setRxTxIrCode({ RxIrRemoteCode: [] })
  }

  const handleConvert = () => {
    if ("RxIrRemoteCode" in rx_tx_ircode) {
      let new_text = "{" + rx_tx_ircode.RxIrRemoteCode.map(item => item.mark + "," + item.space) + "}"
      setState({ ...state, text: new_text })
      message.info('表現を変換しました。')
    } else if ("TxIrRemoteCode" in rx_tx_ircode) {
      var new_text = ""
      var msg = '表現を変換しました。'
      invoke<string>("encode", { input: rx_tx_ircode })
        .then(x => { new_text = x })
        .catch(err => msg = "変換に失敗しました。：" + err)
      setState({ ...state, text: new_text })
      message.info(msg)
    } else {
      throw new Error('unimplemented')
    }
  }

  const handleParse = (text: string) => {
    setState({ ...state, text: text })
    invoke<RxIrRemoteCode>("parse_infrared_code", { ircode: text })
      .then((rx) => {
        setRxTxIrCode({ RxIrRemoteCode: rx })
        setState(state => ({ ...state, alert: { type: "success", message: "いいですね。" } }))
      }).catch(err => setState(state => ({ ...state, alert: { type: 'error', message: err } })))
  }

  return (
    <Space direction="vertical" size="middle" style={{ display: 'flex' }}>
      <Card size="small" title={<Title level={4}>解析する赤外線リモコン信号</Title>}>
        <Button type="primary" style={{ margin: 3 }} onClick={handleReset}>Reset</Button>
        <Button type="default" style={{ margin: 3 }} onClick={handleConvert}>変換</Button>
        <TextArea
          rows={6}
          placeholder="ここに解析対象の赤外線リモコンコードを入れる。"
          value={state.text}
          onChange={(e) => { handleParse(e.target.value) }}
        />
        <Alert message={state.alert.message} type={state.alert.type} showIcon />
      </Card>
      <InfraredRemoteSignal rx_ircode={convert_to_RxIrRemoteCode(rx_tx_ircode)} />
      <IrBitStream rx_tx_ircode={rx_tx_ircode} />
    </Space>
  );
}

export default App;