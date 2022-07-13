// Copyright (c) 2022 Akihiro Yamamoto.
// Licensed under the MIT License <https://spdx.org/licenses/MIT.html>
// See LICENSE file in the project root for full license information.
//
use crate::infrared_remote::{IrCarrierCounter, MarkAndSpaceIrCarrier};
use nom::{
    bytes::complete::take_while_m_n,
    character::complete::multispace0,
    combinator::map_res,
    multi::many1,
    sequence::{delimited, tuple},
    Finish, IResult,
};

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

// 2桁の16進数(8ビット)
fn two_digits_hexadecimal(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

// 4桁の16進数(16ビット)
fn four_digits_hexadecimal(input: &str) -> IResult<&str, u16> {
    let (input, (lower, higher)) = tuple((two_digits_hexadecimal, two_digits_hexadecimal))(input)?;
    // 入力値は 下位8ビット -> 上位8ビット の順番なので普通の数字の書き方(高位が前, 下位が後)に入れ替える。
    Ok((input, ((higher as u16) << 8) | lower as u16))
}

// マークアンドスペース
fn mark_and_space(input: &str) -> IResult<&str, MarkAndSpaceIrCarrier> {
    let (input, (m, s)) = tuple((four_digits_hexadecimal, four_digits_hexadecimal))(input)?;
    Ok((
        input,
        MarkAndSpaceIrCarrier {
            mark: IrCarrierCounter(m),
            space: IrCarrierCounter(s),
        },
    ))
}

// マークアンドスペースのベクタ
fn take_mark_and_spaces(input: &str) -> IResult<&str, Vec<MarkAndSpaceIrCarrier>> {
    many1(delimited(multispace0, mark_and_space, multispace0))(input)
}

// 入力文字列を解析してマークアンドスペースのベクタにする
pub fn from_infrared_code(input: &str) -> Result<Vec<MarkAndSpaceIrCarrier>, String> {
    match take_mark_and_spaces(input).finish() {
        Ok((_, res)) => Ok(res),
        Err(e) => Err(e.to_string()),
    }
}

#[test]
fn test_four_digits_hexadecimal() {
    assert_eq!(four_digits_hexadecimal("5601"), Ok(("", 0x0156)));
    assert_eq!(
        mark_and_space("5601AA00"),
        Ok((
            "",
            MarkAndSpaceIrCarrier {
                mark: IrCarrierCounter(0x0156),
                space: IrCarrierCounter(0x00AA),
            }
        ))
    );
}

#[test]
fn test_take_mark_and_spaces() {
    assert_eq!(
        take_mark_and_spaces("5601AA00 17001500"),
        Ok((
            "",
            vec!(
                (IrCarrierCounter(0x0156), IrCarrierCounter(0x00AA)).into(),
                (IrCarrierCounter(0x0017), IrCarrierCounter(0x0015)).into(),
            )
        ))
    );
}

#[test]
fn test_from_infrared_code() {
    assert_eq!(
        from_infrared_code("5601AA00 17001500"),
        Ok(vec!(
            MarkAndSpaceIrCarrier {
                mark: IrCarrierCounter(0x0156),
                space: IrCarrierCounter(0x00AA),
            },
            MarkAndSpaceIrCarrier {
                mark: IrCarrierCounter(0x0017),
                space: IrCarrierCounter(0x0015),
            }
        ))
    );
}
