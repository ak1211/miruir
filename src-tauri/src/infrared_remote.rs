// Copyright (c) 2022 Akihiro Yamamoto.
// Licensed under the MIT License <https://spdx.org/licenses/MIT.html>
// See LICENSE file in the project root for full license information.
//
use serde::{Deserialize, Serialize};
use std::convert;
use std::fmt;
use std::iter;
use std::ops;
use std::ops::Range;

/// 家製協プロトコルの定義
mod protocol_aeha {
    use crate::infrared_remote::{MarkAndSpace, MarkAndSpaceMicros, Microseconds};
    /// 基準時間 350us ～ 500us typical 425. T = 440 μ秒(実測)
    pub const TIME_BASE: Microseconds = Microseconds(440);

    /// リーダーパルス
    /// H-level width, 8 * T(425us) = typical 3400us
    /// L-level width, 4 * T(425us) = typical 1700us
    pub const LEADER: MarkAndSpaceMicros = MarkAndSpace {
        mark: Microseconds(8 * TIME_BASE.0),
        space: Microseconds(4 * TIME_BASE.0),
    };

    /// 0を意味する信号
    /// H-level width, 1 * T(425us) = typical 425us
    /// L-level width, 1 * T(425us) = typical 425us
    pub const TYPICAL_BIT_ZERO: MarkAndSpaceMicros = MarkAndSpace {
        mark: TIME_BASE,
        space: TIME_BASE,
    };

    /// 1を意味する信号
    /// H-level width, 1 * T(425us) = typical 425us
    /// L-level width, 3 * T(425us) = typical 1275us
    pub const TYPICAL_BIT_ONE: MarkAndSpaceMicros = MarkAndSpace {
        mark: TIME_BASE,
        space: Microseconds(3 * TIME_BASE.0),
    };
}

/// NECプロトコルの定義
mod protocol_nec {
    use crate::infrared_remote::{MarkAndSpace, MarkAndSpaceMicros, Microseconds};
    /// 基準時間 T = 562 μ秒
    pub const TIME_BASE: Microseconds = Microseconds(562);

    /// リーダーパルス
    /// H-level width, 16 * T(562us) = typical 8992us
    /// L-level width, 8 * T(562us) = typical 4496us
    pub const LEADER: MarkAndSpaceMicros = MarkAndSpace {
        mark: Microseconds(16 * TIME_BASE.0),
        space: Microseconds(8 * TIME_BASE.0),
    };

    /// 0を意味する信号
    /// H-level width, 1 * T(562us) = typical 562us
    /// L-level width, 1 * T(562us) = typical 562us
    pub const TYPICAL_BIT_ZERO: MarkAndSpaceMicros = MarkAndSpace {
        mark: TIME_BASE,
        space: TIME_BASE,
    };

    /// 1を意味する信号
    /// H-level width, 1 * T(562us) = typical 562us
    /// L-level width, 3 * T(562us) = typical 1686us
    pub const TYPICAL_BIT_ONE: MarkAndSpaceMicros = MarkAndSpace {
        mark: TIME_BASE,
        space: Microseconds(3 * TIME_BASE.0),
    };
}

/// SIRCプロトコルの定義
mod protocol_sirc {
    use crate::infrared_remote::{MarkAndSpace, MarkAndSpaceMicros, Microseconds};
    /// 基準時間 T = 600 μ秒
    pub const TIME_BASE: Microseconds = Microseconds(600);

    /// リーダーパルス
    /// H-level width, 4 * T(600us) = typical 2400us
    /// L-level width, 1 * T(600us) = typical 600us
    pub const LEADER: MarkAndSpaceMicros = MarkAndSpace {
        mark: Microseconds(4 * TIME_BASE.0),
        space: Microseconds(1 * TIME_BASE.0),
    };

    /// 0を意味する信号
    /// H-level width, 1 * T(600us) = typical 600us
    /// L-level width, 1 * T(600us) = typical 600us
    pub const TYPICAL_BIT_ZERO: MarkAndSpaceMicros = MarkAndSpace {
        mark: TIME_BASE,
        space: TIME_BASE,
    };

    /// 1を意味する信号
    /// H-level width, 2 * T(600us) = typical 1200us
    /// L-level width, 1 * T(600us) = typical 600us
    pub const TYPICAL_BIT_ONE: MarkAndSpaceMicros = MarkAndSpace {
        mark: Microseconds(2 * TIME_BASE.0),
        space: TIME_BASE,
    };
}

/// ずれ時間の許容範囲はとりあえず 300us
const TOLERANCE: Microseconds = Microseconds(300);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// マイクロ秒型
pub struct Microseconds(pub u32);

impl ops::Add for Microseconds {
    type Output = Microseconds;
    /// マイクロ秒型の加算演算子
    fn add(self, other: Self) -> Self::Output {
        Microseconds(self.0 + other.0)
    }
}

impl ops::Sub for Microseconds {
    type Output = Microseconds;
    /// マイクロ秒型の減算演算子
    fn sub(self, other: Self) -> Self::Output {
        Microseconds(self.0 - other.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// 赤外線リモコン信号のキャリア周波数カウンタ型
pub struct IrCarrierCounter(pub u16);

impl IrCarrierCounter {
    /// 16ビットリトルエンディアンで出力する
    /// ```
    /// use infrared_remote::infrared_remote::IrCarrierCounter;
    /// assert_eq!(IrCarrierCounter(0x1234).to_string_littel_endian_u16(), "3412");
    /// assert_eq!(IrCarrierCounter(0xabcd).to_string_littel_endian_u16(), "CDAB");
    /// assert_eq!(IrCarrierCounter(0xf0a0).to_string_littel_endian_u16(), "A0F0");
    /// assert_eq!(IrCarrierCounter(0xa0f0).to_string_littel_endian_u16(), "F0A0");
    /// assert_eq!(IrCarrierCounter(0xff00).to_string_littel_endian_u16(), "00FF");
    /// assert_eq!(IrCarrierCounter(0x00ff).to_string_littel_endian_u16(), "FF00");
    /// ```
    pub fn to_string_littel_endian_u16(self) -> String {
        let upper = (self.0 >> 8) & 0xff;
        let lower = self.0 & 0xff;
        format!("{lower:02X}{upper:02X}")
    }
}

impl ops::Add for IrCarrierCounter {
    type Output = IrCarrierCounter;
    /// 赤外線リモコン信号のキャリア周波数カウンタ型の加算演算子
    fn add(self, other: Self) -> Self::Output {
        IrCarrierCounter(self.0 + other.0)
    }
}

impl ops::Sub for IrCarrierCounter {
    type Output = IrCarrierCounter;
    /// 赤外線リモコン信号のキャリア周波数カウンタ型の減算演算子
    fn sub(self, other: Self) -> Self::Output {
        IrCarrierCounter(self.0 - other.0)
    }
}

impl fmt::Display for IrCarrierCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// 赤外線リモコン信号のキャリア周波数
/// 38000 Hz = 38 kHz
pub const IR_CARRIER_FREQ: u16 = 38000;

impl convert::From<IrCarrierCounter> for Microseconds {
    /// 赤外線リモコン信号のキャリア周波数カウンタ型からマイクロ秒型へ変換する
    fn from(x: IrCarrierCounter) -> Self {
        // 1 カウント が 1/IR_CARRIER_FREQ 秒 なので
        // 1000倍してミリ秒に
        // さらに1000倍してマイクロ秒にする
        let y = 1_000_000u64 * x.0 as u64 / IR_CARRIER_FREQ as u64;
        Self(y as u32)
    }
}

impl convert::From<Microseconds> for IrCarrierCounter {
    /// マイクロ秒型から赤外線リモコン信号のキャリア周波数カウンタ型へ変換する
    fn from(x: Microseconds) -> Self {
        // 1 秒が IR_CARRIER_FREQ カウントなので
        // 1マイクロ秒 が IrCarrirFreq/(1000*1000) カウント
        let y = x.0 as u64 * IR_CARRIER_FREQ as u64 / 1_000_000u64;
        Self(y as u16)
    }
}

#[test]
fn test_microseconds_to_ircarriercounter() {
    assert_eq!(
        MarkAndSpaceIrCarrier::from(MarkAndSpaceMicros {
            mark: Microseconds(9000),
            space: Microseconds(4500),
        }),
        MarkAndSpaceIrCarrier {
            mark: IrCarrierCounter(0x0156),
            space: IrCarrierCounter(0x00AB),
        },
    );
    assert_eq!(
        IrCarrierCounter::from(Microseconds(4500)),
        IrCarrierCounter(0x00AB)
    );
    assert_eq!(
        IrCarrierCounter::from(Microseconds(9000)),
        IrCarrierCounter(0x0156)
    );
    assert_eq!(
        IrCarrierCounter::from(Microseconds(4500)),
        IrCarrierCounter(0x00AB)
    );
    assert_eq!(
        IrCarrierCounter::from(Microseconds(3400)),
        IrCarrierCounter(0x0081)
    );
    assert_eq!(
        IrCarrierCounter::from(Microseconds(1700)),
        IrCarrierCounter(0x0040)
    );
    assert_eq!(
        IrCarrierCounter::from(Microseconds(2400)),
        IrCarrierCounter(0x005B)
    );
    assert_eq!(
        IrCarrierCounter::from(Microseconds(600)),
        IrCarrierCounter(0x0016)
    );
}

#[test]
fn test_ircarriercounter_to_microseconds() {
    assert_eq!(
        MarkAndSpaceMicros::from(MarkAndSpaceIrCarrier {
            mark: IrCarrierCounter(0x0156),
            space: IrCarrierCounter(0x00AB),
        }),
        MarkAndSpaceMicros {
            mark: Microseconds(9000),
            space: Microseconds(4500),
        },
    );
    assert_eq!(
        Microseconds::from(IrCarrierCounter(0x0156)),
        Microseconds(9000)
    );
    assert_eq!(
        Microseconds::from(IrCarrierCounter(0x00AB)),
        Microseconds(4500)
    );
    assert_eq!(
        Microseconds::from(IrCarrierCounter(0x0081)),
        Microseconds(3394)
    );
    assert_eq!(
        Microseconds::from(IrCarrierCounter(0x0040)),
        Microseconds(1684)
    );
    assert_eq!(
        Microseconds::from(IrCarrierCounter(0x005B)),
        Microseconds(2394)
    );
    assert_eq!(
        Microseconds::from(IrCarrierCounter(0x0016)),
        Microseconds(578)
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// マークアンドスペース型
pub struct MarkAndSpace<T> {
    pub mark: T,
    pub space: T,
}

impl<T> convert::From<MarkAndSpace<T>> for (T, T) {
    /// マークアンドスペース型からタプル型へ変換する
    fn from(x: MarkAndSpace<T>) -> Self {
        (x.mark, x.space)
    }
}

impl<T> convert::From<(T, T)> for MarkAndSpace<T> {
    /// タプル型からマークアンドスペース型へ変換する
    fn from((a, b): (T, T)) -> Self {
        MarkAndSpace { mark: a, space: b }
    }
}

impl<T: ops::Add<Output = T>> ops::Add for MarkAndSpace<T> {
    type Output = MarkAndSpace<T>;
    /// マークアンドスペースの加算演算子
    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            mark: self.mark + other.mark,
            space: self.space + other.space,
        }
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for MarkAndSpace<T> {
    type Output = MarkAndSpace<T>;
    /// マークアンドスペースの減算演算子
    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            mark: self.mark - other.mark,
            space: self.space - other.space,
        }
    }
}

/// マークアンドスペース(マイクロ秒ベース)
pub type MarkAndSpaceMicros = MarkAndSpace<Microseconds>;

/// マークアンドスペース(キャリア周波数カウンタ型ベース)
pub type MarkAndSpaceIrCarrier = MarkAndSpace<IrCarrierCounter>;

impl MarkAndSpaceIrCarrier {
    /// 16ビットリトルエンディアンで出力する
    pub fn to_string_littel_endian_u16(&self) -> String {
        format!(
            "{}{}",
            IrCarrierCounter::from(self.mark).to_string_littel_endian_u16(),
            IrCarrierCounter::from(self.space).to_string_littel_endian_u16()
        )
    }
}

impl convert::From<MarkAndSpaceIrCarrier> for MarkAndSpaceMicros {
    /// マークアンドスペース(キャリア周波数カウンタ型ベース)から
    /// マークアンドスペース(マイクロ秒ベース)へ
    /// 変換する
    fn from(carrir: MarkAndSpaceIrCarrier) -> Self {
        Self {
            mark: carrir.mark.into(),
            space: carrir.space.into(),
        }
    }
}

impl convert::From<MarkAndSpaceMicros> for MarkAndSpaceIrCarrier {
    /// マークアンドスペース(マイクロ秒ベース)から
    /// マークアンドスペース(キャリア周波数カウンタ型ベース)へ
    /// 変換する
    fn from(micros: MarkAndSpaceMicros) -> Self {
        Self {
            mark: micros.mark.into(),
            space: micros.space.into(),
        }
    }
}

/// 第1,2,3...フレームを区切る時間(8ms = 8000us)
pub const THRESHOLD_FRAME_GAP: Microseconds = Microseconds(8000);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// 赤外線リモコン信号フレーム
pub struct InfraredRemoteFrame(Vec<MarkAndSpaceMicros>);

impl InfraredRemoteFrame {
    /// 先頭
    pub fn head(&self) -> MarkAndSpaceMicros {
        self.0[0]
    }
    /// 残り
    pub fn tail(&self) -> &[MarkAndSpaceMicros] {
        &self.0[1..]
    }
}

impl iter::FromIterator<MarkAndSpaceMicros> for InfraredRemoteFrame {
    fn from_iter<T: IntoIterator<Item = MarkAndSpaceMicros>>(iter: T) -> Self {
        let mut c = InfraredRemoteFrame(Vec::new());
        for i in iter {
            c.0.push(i);
        }
        c
    }
}

/// デコード1段階目
/// 入力マークアンドスペース列を各フレームに分ける
pub fn decode_phase1(input: &[MarkAndSpaceMicros]) -> Result<Vec<InfraredRemoteFrame>, String> {
    if input.len() < 1 {
        return Err("decode_phase1: input is empty.".to_string());
    }

    let threshold = THRESHOLD_FRAME_GAP;
    let xs = input.split_inclusive(|ms| threshold <= ms.space);

    let mut result = Vec::new();
    for x in xs {
        result.push(InfraredRemoteFrame(x.to_vec()));
    }
    Ok(result)
}

#[test]
fn test1_decode_phase1() {
    assert_eq!(
        decode_phase1(&vec!()),
        Err("decode_phase1: input is empty.".to_string())
    )
}

#[test]
#[should_panic]
fn test2_decode_phase1() {
    use crate::parsing::from_infrared_code;
    let a = from_infrared_code("").unwrap();
    let b = a
        .iter()
        .map(|x| MarkAndSpaceMicros::from(*x))
        .collect::<Vec<MarkAndSpaceMicros>>();
    let _ = decode_phase1(&b);
}

#[test]
fn test3_decode_phase1() {
    use crate::parsing::from_infrared_code;
    let a = from_infrared_code("5601AA00 17001500 18001400 18001500").unwrap();
    let b = a
        .iter()
        .map(|x| MarkAndSpaceMicros::from(*x))
        .collect::<Vec<MarkAndSpaceMicros>>();
    assert_eq!(
        decode_phase1(&b),
        Ok(vec!(InfraredRemoteFrame(vec!(
            MarkAndSpaceIrCarrier {
                mark: IrCarrierCounter(0x0156),
                space: IrCarrierCounter(0x00AA),
            }
            .into(),
            MarkAndSpaceIrCarrier {
                mark: IrCarrierCounter(0x0017),
                space: IrCarrierCounter(0x0015),
            }
            .into(),
            MarkAndSpaceIrCarrier {
                mark: IrCarrierCounter(0x0018),
                space: IrCarrierCounter(0x0014),
            }
            .into(),
            MarkAndSpaceIrCarrier {
                mark: IrCarrierCounter(0x0018),
                space: IrCarrierCounter(0x0015),
            }
            .into(),
        ))))
    );
}

#[test]
fn test4_decode_phase1() {
    use crate::parsing::from_infrared_code;
    let a = &from_infrared_code("5601AA00 17001500 18001400 18001500").unwrap();
    let b = a
        .iter()
        .map(|x| MarkAndSpaceMicros::from(*x))
        .collect::<Vec<MarkAndSpaceMicros>>();
    assert_eq!(
        decode_phase1(&b),
        Ok(vec!(InfraredRemoteFrame(vec!(
            MarkAndSpaceIrCarrier {
                mark: IrCarrierCounter(0x0156),
                space: IrCarrierCounter(0x00AA),
            }
            .into(),
            MarkAndSpaceIrCarrier {
                mark: IrCarrierCounter(0x0017),
                space: IrCarrierCounter(0x0015),
            }
            .into(),
            MarkAndSpaceIrCarrier {
                mark: IrCarrierCounter(0x0018),
                space: IrCarrierCounter(0x0014),
            }
            .into(),
            MarkAndSpaceIrCarrier {
                mark: IrCarrierCounter(0x0018),
                space: IrCarrierCounter(0x0015),
            }
            .into(),
        ))))
    );
}

#[test]
fn test5_decode_phase1() {
    use crate::parsing::from_infrared_code;
    let a = from_infrared_code("5601AA00 17008001 5601AA00 18001500").unwrap();
    let b = a
        .iter()
        .map(|x| MarkAndSpaceMicros::from(*x))
        .collect::<Vec<MarkAndSpaceMicros>>();
    assert_eq!(
        decode_phase1(&b),
        Ok(vec!(
            InfraredRemoteFrame(vec!(
                MarkAndSpaceIrCarrier {
                    mark: IrCarrierCounter(0x0156),
                    space: IrCarrierCounter(0x00AA),
                }
                .into(),
                MarkAndSpaceIrCarrier {
                    mark: IrCarrierCounter(0x0017),
                    space: IrCarrierCounter(0x0180),
                }
                .into(),
            )),
            InfraredRemoteFrame(vec!(
                MarkAndSpaceIrCarrier {
                    mark: IrCarrierCounter(0x0156),
                    space: IrCarrierCounter(0x00AA),
                }
                .into(),
                MarkAndSpaceIrCarrier {
                    mark: IrCarrierCounter(0x0018),
                    space: IrCarrierCounter(0x0015),
                }
                .into()
            ))
        ),)
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// 1ビットを表す型
pub struct Bit(u8);

impl Bit {
    //
    pub fn new(init: u8) -> Self {
        match init {
            0 => Self(0),
            1 => Self(1),
            _ => panic!("0 か 1 でよろしく"),
        }
    }
}

/// ビット型の配列を8ビットごとに空白を入れて表示する。
fn show_bit_pattern(input: &[Bit]) -> String {
    let mut s = String::new();
    for (index, item) in input.iter().enumerate() {
        s.push(match item {
            Bit(0) => '0',
            Bit(1) => '1',
            _ => 'X',
        });
        if index & 7 == 7 {
            s.push(' ');
        }
    }
    s
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// 復調後の赤外線リモコン信号
pub enum InfraredRemoteDemodulatedFrame {
    Aeha(Vec<Bit>),
    Nec(Vec<Bit>),
    Sirc(Vec<Bit>),
    Unknown(Vec<MarkAndSpaceMicros>),
}

impl fmt::Display for InfraredRemoteDemodulatedFrame {
    //
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfraredRemoteDemodulatedFrame::Aeha(x) => write!(f, "AEHA {}", show_bit_pattern(x)),
            InfraredRemoteDemodulatedFrame::Nec(x) => write!(f, "NEC {}", show_bit_pattern(x)),
            InfraredRemoteDemodulatedFrame::Sirc(x) => write!(f, "SIRC {}", show_bit_pattern(x)),
            InfraredRemoteDemodulatedFrame::Unknown(x) => write!(f, "Unknown {:?}", x),
        }
    }
}

/// デコード2段階目
/// 入力信号を復調して赤外線リモコン信号を取り出す
pub fn decode_phase2(input: &InfraredRemoteFrame) -> InfraredRemoteDemodulatedFrame {
    /// pulse distance modulation: NEC, AEHA
    fn demodulate_pulse_distance_modulation(x: MarkAndSpaceMicros) -> Bit {
        if x.mark + x.mark <= x.space {
            // マーク時間の２倍以上スペース時間があれば
            Bit(1)
        } else {
            Bit(0)
        }
    }
    /// pulse width modulation: SIRC
    fn demodulate_pulse_width_modulation(x: MarkAndSpaceMicros) -> Bit {
        // upper lower tolerance 0.1ms = 100us
        let tolerance = Microseconds(100);
        let threshold = Microseconds(1200);
        let upper = threshold + tolerance;
        let lower = threshold - tolerance;
        if lower <= x.mark && x.mark <= upper {
            // マーク時間が閾値(1200us)付近なら
            Bit(1)
        } else {
            Bit(0)
        }
    }
    //
    let aeha = (
        Range {
            start: protocol_aeha::LEADER.mark - TOLERANCE,
            end: protocol_aeha::LEADER.mark + TOLERANCE,
        },
        Range {
            start: protocol_aeha::LEADER.space - TOLERANCE,
            end: protocol_aeha::LEADER.space + TOLERANCE,
        },
    );
    //
    let nec = (
        Range {
            start: protocol_nec::LEADER.mark - TOLERANCE,
            end: protocol_nec::LEADER.mark + TOLERANCE,
        },
        Range {
            start: protocol_nec::LEADER.space - TOLERANCE,
            end: protocol_nec::LEADER.space + TOLERANCE,
        },
    );
    //
    let sirc = (
        Range {
            start: protocol_sirc::LEADER.mark - TOLERANCE,
            end: protocol_sirc::LEADER.mark + TOLERANCE,
        },
        Range {
            start: protocol_sirc::LEADER.space - TOLERANCE,
            end: protocol_sirc::LEADER.space + TOLERANCE,
        },
    );
    //
    let leader_pulse = input.head();
    let tail = &input.tail();
    //
    fn compare(
        test: MarkAndSpaceMicros,
        (mark, space): (Range<Microseconds>, Range<Microseconds>),
    ) -> bool {
        mark.contains(&test.mark) && space.contains(&test.space)
    }
    //
    if compare(leader_pulse, aeha) {
        // PDM復調する
        InfraredRemoteDemodulatedFrame::Aeha(
            tail.iter()
                .map(|&x| demodulate_pulse_distance_modulation(x))
                .collect(),
        )
    } else if compare(leader_pulse, nec) {
        // PDM復調する
        InfraredRemoteDemodulatedFrame::Nec(
            tail.iter()
                .map(|&x| demodulate_pulse_distance_modulation(x))
                .collect(),
        )
    } else if compare(leader_pulse, sirc) {
        // PWM復調する
        InfraredRemoteDemodulatedFrame::Sirc(
            tail.iter()
                .map(|&x| demodulate_pulse_width_modulation(x))
                .collect(),
        )
    } else {
        InfraredRemoteDemodulatedFrame::Unknown(input.0.clone())
    }
}

/// エンコード1段階目
/// 赤外線リモコン信号から変調済みフレームを組み立てる
pub fn encode_phase1(
    input: &InfraredRemoteDemodulatedFrame,
) -> Result<InfraredRemoteFrame, String> {
    /// 家製協プロトコルに従ってビット列を変調する
    fn modulate_aeha(x: &Bit) -> Result<MarkAndSpaceMicros, String> {
        match x {
            Bit(0) => Ok(protocol_aeha::TYPICAL_BIT_ZERO),
            Bit(1) => Ok(protocol_aeha::TYPICAL_BIT_ONE),
            _ => Err(format!("encode_phase1: 異常値 {:?} です。", x)),
        }
    }
    /// NECプロトコルに従ってビット列を変調する
    fn modulate_nec(x: &Bit) -> Result<MarkAndSpaceMicros, String> {
        match x {
            Bit(0) => Ok(protocol_nec::TYPICAL_BIT_ZERO),
            Bit(1) => Ok(protocol_nec::TYPICAL_BIT_ONE),
            _ => Err(format!("encode_phase1: 異常値 {:?} です。", x)),
        }
    }
    /// SIRCプロトコルに従ってビット列を変調する
    fn modulate_sirc(x: &Bit) -> Result<MarkAndSpaceMicros, String> {
        match x {
            Bit(0) => Ok(protocol_sirc::TYPICAL_BIT_ZERO),
            Bit(1) => Ok(protocol_sirc::TYPICAL_BIT_ONE),
            _ => Err(format!("encode_phase1: 異常値 {:?} です。", x)),
        }
    }
    match input {
        InfraredRemoteDemodulatedFrame::Aeha(bitstream) => {
            let leader = protocol_aeha::LEADER;
            let trailer = bitstream
                .iter()
                .map(|bit| modulate_aeha(bit))
                .collect::<Result<Vec<MarkAndSpaceMicros>, _>>()?;
            // リーダーパルスを復元する
            Ok(InfraredRemoteFrame([vec![leader], trailer].concat()))
        }
        InfraredRemoteDemodulatedFrame::Nec(bitstream) => {
            let leader = protocol_nec::LEADER;
            let trailer = bitstream
                .iter()
                .map(|bit| modulate_nec(bit))
                .collect::<Result<Vec<MarkAndSpaceMicros>, _>>()?;
            // リーダーパルスを復元する
            Ok(InfraredRemoteFrame([vec![leader], trailer].concat()))
        }
        InfraredRemoteDemodulatedFrame::Sirc(bitstream) => {
            let leader = protocol_sirc::LEADER;
            let trailer = bitstream
                .iter()
                .map(|bit| modulate_sirc(bit))
                .collect::<Result<Vec<MarkAndSpaceMicros>, _>>()?;
            // リーダーパルスを復元する
            Ok(InfraredRemoteFrame([vec![leader], trailer].concat()))
        }
        InfraredRemoteDemodulatedFrame::Unknown(_) => Err("encode_phase1: 不明プロトコルでは変調できません。".to_string()),
    }
}

/// エンコード2段階目
/// 変調済みフレームフレームを結合してマークアンドスペースにする
pub fn encode_phase2(input: &[InfraredRemoteFrame]) -> Vec<MarkAndSpaceMicros> {
    let mut frames: Vec<InfraredRemoteFrame> = Vec::new();

    // 各フレームの最終スペース時間を THRESHOLD_FRAME_GAP に変換する。
    for item in input {
        let mut x = item.clone();
        if let Some(last) = x.0.last_mut() {
            // 最終フレーム
            *last = MarkAndSpaceMicros {
                mark: last.mark,
                space: THRESHOLD_FRAME_GAP,
            };
        }
        frames.push(x);
    }
    frames.into_iter().flat_map(|x| x.0).collect()
}

/// 赤外線リモコン信号からマークアンドスペースにする
pub fn encode_to_mark_and_spaces(
    input: &[InfraredRemoteDemodulatedFrame],
) -> Result<Vec<MarkAndSpaceMicros>, String> {
    let frames = input
        .iter()
        .map(|x| encode_phase1(x))
        .collect::<Result<Vec<InfraredRemoteFrame>, String>>()?;
    Ok(encode_phase2(&frames))
}

/// エンコード3段階目
/// マークアンドスペースのベクタを送信形式に
pub fn encode_phase3(input: &[MarkAndSpaceMicros]) -> String {
    input
        .iter()
        .map(|v| MarkAndSpaceIrCarrier::from(*v).to_string_littel_endian_u16())
        .collect()
}

/// 送信する赤外線リモコン信号を得る
pub fn encode_infrared_remote_code(
    input: &[InfraredRemoteDemodulatedFrame],
) -> Result<String, String> {
    encode_to_mark_and_spaces(input).map(|v| encode_phase3(&v))
}

#[cfg(test)]
mod decode_tests {
    use crate::infrared_remote::{
        decode_phase1, decode_phase2, encode_infrared_remote_code, InfraredRemoteDemodulatedFrame,
        MarkAndSpaceMicros,
    };
    use crate::parsing::from_infrared_code;

    fn decode(input: &str) -> Result<Vec<InfraredRemoteDemodulatedFrame>, String> {
        let a = from_infrared_code(input)?;
        let markandspaces = a
            .iter()
            .map(|x| MarkAndSpaceMicros::from(*x))
            .collect::<Vec<MarkAndSpaceMicros>>();
        let frames = decode_phase1(&markandspaces)?;
        Ok(frames
            .iter()
            .map(|frame| decode_phase2(frame))
            .collect::<Vec<InfraredRemoteDemodulatedFrame>>())
    }

    #[test]
    fn test1_decode() {
        let ircode= "8600420014000F00130031001300100014000E0014000F001300100014000F001300100014000F00130010001300100013000F001300100013003100130010001300100013000F0014000F0013001000130010001300100013001000130010001300300014000F0013001000130010001300100013000F0014000F001300100014000F00140030001300100013003000140030001300310014002F0014000F0015000D0016003000130010001300300014003000130031001300300014000F001300310014004F03";
        let codes = decode(ircode).unwrap();
        for item in codes {
            println!("{:?}", item);
        }
    }

    #[test]
    fn test2_decode() {
        let ircode= "8700410015000F00130030001300100014000E0015000F0013000F0014000F0014000F00130010001400100013000F0014000F0014000E001400300014000F001400100013000E0015000E0015000F0013000F0014000F001400100013000E0015002F00140010001300100013000F001300110013000E0014000F0015000F0013001000130030001400100013002F001400100013000F00140010001300100013000E0015002F0014000F001400300014000E0015000E0015000F00120010001400310013004F03";
        let codes = decode(ircode).unwrap();
        for item in codes {
            println!("{}", item);
        }
    }

    #[test]
    fn test3_decode() {
        let ircode= "5601A900180015001800140018001400190013001900140019001400170040001700150018003F0019003E0018003E0019003F0019003E00170040001800140019003E001800150018003F00180014001800140019003F0018001400170016001700150018003F001800140018003F0018003F001800140019003F0018003F0018003E0019004F03";
        let codes = decode(ircode).unwrap();
        for item in codes {
            println!("{}", item);
        }
    }

    #[test]
    fn test4_decode() {
        let ircode= "5601AA0017001500180014001800150018001400170016001700150018003F0018001400180040001700400017003F001800400018003F0017003F001800150018003F001800150018003E0018003F001700410017003F0019003E00180015001700160016004000180014001800150018001500170016001600160017003F0018003F0018004F03";
        let codes = decode(ircode).unwrap();
        for item in codes {
            println!("{}", item);
        }
    }

    #[test]
    fn test5_decode() {
        let ircode= "5B0019002D001900170018002E001800170018002E00180017001800170019002E0018002E0018002E001800170019002D001900160019002D001900170018001700180017001900170018001700180017000B025C0018002E001700180018002E001800180017002F0017001800170018001700300017002F0016002F001800170018002E001700180019002D001800180017001800180017001800170018001800180017000B025C0018002E001800170018002E001800170018002E0018001800170018001700300017002F0017002F001700180017002F001700180019002D001700180018001800170018001700180018001800170018000B025C0018002E001800180017002F001700180017002F00170019001600180018002E0019002E0017002F001700180018002E001800180017002F001700180017001800170018001900170017001800180017004F03";
        let codes = decode(ircode).unwrap();
        for item in codes {
            println!("{}", item);
        }
    }

    #[test]
    fn test6_decode() {
        let ircode= "5B0018002E001800180018002E001800170018002F00170018001700180017002F00180017001900170018001700180018004F03";
        let codes = decode(ircode).unwrap();
        for item in codes {
            println!("{}", item);
        }
    }

    #[test]
    fn test7_decode() {
        let ircode= "1100120012001100120011001100120012001100120011001200B0038200450011003300120011001100120011001200110033001200110011001200120011001200110012003200120011001100330012003200120011001200320011003300120032001200320012003200120011001200110013003100130010001200110012001100120011001100120012001100120011001100120012001100110012001200320011001200120032001200110012001100120011001200320011003300120013000F0012001200110012001100110033001200110012003200130010001100120012001100120011001200110011001200110012001200110012001100110033001200320011003300110012001200110011003300120011001100120012004F03";
        let codes = decode(ircode).unwrap();
        for item in codes {
            println!("{}", item);
        }
    }

    #[test]
    fn test1_decode_and_encode() {
        let ircode="5601A900180015001800140018001400190013001900140019001400170040001700150018003F0019003E0018003E0019003F0019003E00170040001800140019003E001800150018003F00180014001800140019003F0018001400170016001700150018003F001800140018003F0018003F001800140019003F0018003F0018003E0019004F03";
        let frames = decode(ircode).unwrap();
        //
        print!(
            "decoded\n{}",
            frames
                .iter()
                .map(|x| format!("{}\n", x))
                .collect::<String>()
        );
        //
        let encoded = encode_infrared_remote_code(&frames).unwrap();
        //
        let frames2 = decode(&encoded).unwrap();
        //
        print!(
            "encoded\n{}",
            frames2
                .iter()
                .map(|x| format!("{}\n", x))
                .collect::<String>()
        );
        //
        assert_eq!(frames, frames2);
    }

    #[test]
    fn test2_decode_and_encode() {
        let ircode= "8600420014000F00130031001300100014000E0014000F001300100014000F001300100014000F00130010001300100013000F001300100013003100130010001300100013000F0014000F0013001000130010001300100013001000130010001300300014000F0013001000130010001300100013000F0014000F001300100014000F00140030001300100013003000140030001300310014002F0014000F0015000D0016003000130010001300300014003000130031001300300014000F001300310014004F03";
        let codes = decode(ircode).unwrap();
        //
        print!(
            "decoded\n{}",
            codes.iter().map(|x| format!("{}\n", x)).collect::<String>()
        );
        //
        let encoded = encode_infrared_remote_code(&codes).unwrap();
        //
        let codes2 = decode(&encoded).unwrap();
        //
        print!(
            "encoded\n{}",
            codes2
                .iter()
                .map(|x| format!("{}\n", x))
                .collect::<String>()
        );
        //
        assert_eq!(codes, codes2);
    }

    #[test]
    fn test3_decode_and_encode() {
        let ircode= "5B0019002D001900170018002E001800170018002E00180017001800170019002E0018002E0018002E001800170019002D001900160019002D001900170018001700180017001900170018001700180017000B025C0018002E001700180018002E001800180017002F0017001800170018001700300017002F0016002F001800170018002E001700180019002D001800180017001800180017001800170018001800180017000B025C0018002E001800170018002E001800170018002E0018001800170018001700300017002F0017002F001700180017002F001700180019002D001700180018001800170018001700180018001800170018000B025C0018002E001800180017002F001700180017002F00170019001600180018002E0019002E0017002F001700180018002E001800180017002F001700180017001800170018001900170017001800180017004F03";
        let codes = decode(ircode).unwrap();
        //
        print!(
            "decoded\n{}",
            codes.iter().map(|x| format!("{}\n", x)).collect::<String>()
        );
        //
        let encoded = encode_infrared_remote_code(&codes).unwrap();
        //
        let codes2 = decode(&encoded).unwrap();
        //
        print!(
            "encoded\n{}",
            codes2
                .iter()
                .map(|x| format!("{}\n", x))
                .collect::<String>()
        );
        //
        assert_eq!(codes, codes2);
    }
}
