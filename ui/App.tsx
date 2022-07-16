// Copyright (c) 2022 Akihiro Yamamoto.
// Licensed under the MIT License <https://spdx.org/licenses/MIT.html>
// See LICENSE file in the project root for full license information.
//
import { useState } from 'react';
import { Button, Card, Divider, Input, Alert, Typography, Space, Table, message } from 'antd';
import 'antd/dist/antd.min.css';
import { Line, Datum } from '@ant-design/charts';
import { invoke } from '@tauri-apps/api/tauri'
import { RxIrRemoteCode, MarkAndSpace } from './index';
import IrBitStream from './IrBitStream';
import './App.css';

const { TextArea } = Input;
const { Text, Title } = Typography;

//
const InfraredRemoteSignal = (props: { rx_ircode: RxIrRemoteCode }): JSX.Element => {
  let mark_and_spaces: MarkAndSpace[] = props.rx_ircode;

  type DatumForList = { sn: number, t: number, kinds: string, duration: number }
  const convert_for_list = (input: MarkAndSpace[]): DatumForList[] => {
    var acc: number = 0;
    var output: DatumForList[] = [];
    input.forEach((item, index) => {
      output.push({ sn: 1 + 2 * index, t: acc, kinds: "Mark", duration: item.mark });
      acc += item.mark;
      output.push({ sn: 2 + 2 * index, t: acc, kinds: "Space", duration: item.space });
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
  const [ircode, setIRCode] = useState<RxIrRemoteCode>([])

  const handleReset = () => {
    setState(initState)
    setIRCode([])
  }

  const handleConvert = () => {
    let newText = "{" + ircode.map(item => item.mark + "," + item.space) + "}"
    setState({ ...state, text: newText })
    message.info('succsessful converting')
  }

  const handleParse = (text: string) => {
    setState({ ...state, text: text })
    invoke<RxIrRemoteCode>("parse_infrared_code", { ircode: text })
      .then((rx) => {
        setIRCode(rx)
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
      <InfraredRemoteSignal rx_ircode={ircode} />
      <IrBitStream rx_ircode={ircode} />
    </Space>
  );
}

export default App;