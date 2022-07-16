// Copyright (c) 2022 Akihiro Yamamoto.
// Licensed under the MIT License <https://spdx.org/licenses/MIT.html>
// See LICENSE file in the project root for full license information.
//
import { useState, useEffect } from 'react'
import './IrBitStream.css'
import { Statistic, Empty, Alert, Card, Divider, Radio, Space, Typography, Descriptions } from 'antd'
import 'antd/dist/antd.min.css'
import { invoke } from '@tauri-apps/api/tauri'
import { RxIrRemoteCode, InfraredRemoteDemodulatedFrame } from './index'

const { Title, Text, Paragraph } = Typography

// オクテット単位にまとめる
const to_octets = (bitstream: Uint8Array): Uint8Array[] => {
  var output: Uint8Array[] = []
  for (let i = 0; i < bitstream.length; i += 8) {
    output.push(bitstream.slice(i, i + 8))
  }
  return output
}

//
const InfraredRemoteFrame = (props: {
  msb_first: boolean,
  index: number,
  frame: InfraredRemoteDemodulatedFrame
}): JSX.Element => {
  var protocol = ""
  var bitstream = new Uint8Array()

  if ("Aeha" in props.frame) {
    protocol = "AEHA"
    bitstream = props.frame.Aeha
  } else if ("Nec" in props.frame) {
    protocol = "NEC"
    bitstream = props.frame.Nec
  } else if ("Sirc" in props.frame) {
    protocol = "SIRC"
    bitstream = props.frame.Sirc
  } else if ("Unknown" in props.frame) {
    protocol = "UNKNOWN"
  } else {
    throw new Error('unimplemented')
  }

  let descriptions_item: JSX.Element[] =
    to_octets(bitstream)
      .map((octets, index) => {
        let xs = props.msb_first ? octets : octets.slice().reverse()
        let value = xs
          .reduce((acc, x) => 2 * acc + x, 0)
          .toString(16)
          .padStart(2, '0')
        let offset = 8 * index
        return (
          <Descriptions.Item label={"offset " + offset} style={{ textAlign: "center" }}>
            <Text> {octets.join('')}</Text>
            <Statistic title="hex" value={value} />
          </Descriptions.Item >
        )
      })

  return (
    <Descriptions
      title={"Frame# " + (1 + props.index)}
      layout="vertical"
      column={{ xxl: 14, xl: 12, lg: 8, md: 6, sm: 2, xs: 1 }}
      bordered>
      <Descriptions.Item key={bitstream.join("")} label={"Bitstream " + bitstream.length + " bits"} span={20}>
        <Text>{bitstream.join('')}</Text>
      </Descriptions.Item>
      <Descriptions.Item label="Protocol" span={1}>
        <Statistic value={protocol} />
      </Descriptions.Item>
      {descriptions_item}
    </Descriptions>
  )
}

type Props = {
  rx_ircode: RxIrRemoteCode,
}

type State = {
  msb_first: number,
  ir_frames: InfraredRemoteDemodulatedFrame[],
  alert: {
    type: 'success' | 'info' | 'warning' | 'error',
    message: string,
  },
};

const initState: State = {
  msb_first: 1,
  ir_frames: [],
  alert: {
    type: 'info',
    message: "",
  },
};

//
const IrBitStream = (props: Props): JSX.Element => {
  const [state, setState] = useState<State>(initState)

  const decode = (ircode: RxIrRemoteCode) => {
    invoke<InfraredRemoteDemodulatedFrame[]>("decode", { input: ircode })
      .then((tx) => {
        setState({
          ...state,
          ir_frames: tx,
          alert: { type: "success", message: "デコード成功" },
        })
      })
      .catch(err => setState(state => ({ ...state, ir_frames: [], message: err })))
  }

  useEffect(
    () => {
      if (props.rx_ircode.length) {
        decode(props.rx_ircode)
      } else {
        setState(state => ({ ...state, ir_frames: [] }))
      }
    }
    , [props.rx_ircode])

  //

  let content =
    <>
      <Space direction='vertical' size="large" style={{ display: 'flex' }}>
        <Paragraph>
          <Text>ビットオーダー&nbsp;</Text>
          <Radio.Group
            name="radiogroup"
            defaultValue={state.msb_first}
            onChange={e => { setState({ ...state, msb_first: e.target.value }) }}
            optionType='button'
            buttonStyle='solid'
          >
            <Radio value={0}>LSBit first</Radio>
            <Radio value={1}>MSBit first</Radio>
          </Radio.Group>
        </Paragraph>
        <Paragraph>
          {(state.msb_first) ?
            <Text>Most Significant Bit (MSB) first<br />
              上位ビット(オクテットのBit7 = 2の7乗 = 128)を先頭に送信されたビット列として解析する。<br />
              先着順で 128, 64, 32, 16, 8, 4, 2, 1の重みづけに対応する。
            </Text>
            :
            <Text>Least Significant Bit (LSB) first<br />
              下位ビット(オクテットのBit0 = 2の0乗 = 1)を先頭に送信されたビット列として解析する。<br />
              先着順で 1, 2, 4, 8, 16, 32, 64, 128の重みづけに対応する。
            </Text>
          }
        </Paragraph>
        {state.ir_frames.map((item, index) =>
          <InfraredRemoteFrame key={index} msb_first={state.msb_first === 1} index={index} frame={item} />
        )}
        <Alert message={state.alert.message} type={state.alert.type} showIcon />
      </Space>
    </>

  if (state.ir_frames.length) {
    return (
      <Card size="small" title={<Title level={4}>ビットストリーム</Title>}>
        {content}
      </Card >
    )
  } else {
    return (
      <Card size="small" title={<Title level={4}>ビットストリーム</Title>}>
        <Empty image={Empty.PRESENTED_IMAGE_SIMPLE} />
      </Card >
    )
  }

}

export default IrBitStream;
