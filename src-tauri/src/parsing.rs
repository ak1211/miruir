use crate::infrared_remote::{
    IrCarrierCounter, MarkAndSpaceIrCarrier, MarkAndSpaceMicros, Microseconds,
};
use nom::{
    bytes::complete::{take_while, take_while_m_n},
    character::complete::{char, multispace0},
    combinator::{map_res, opt},
    multi::many1,
    sequence::{delimited, tuple},
    Finish, IResult,
};
use std::str::FromStr;

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

// 2桁の16進数(8ビット)
fn two_digits_hexadecimal(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

// 4桁の16進数(16ビット)
fn four_digits_hexadecimal(input: &str) -> IResult<&str, IrCarrierCounter> {
    let (input, (lower, higher)) = tuple((two_digits_hexadecimal, two_digits_hexadecimal))(input)?;
    // 入力値は 下位8ビット -> 上位8ビット の順番なので普通の数字の書き方(高位が前, 下位が後)に入れ替える。
    Ok((
        input,
        IrCarrierCounter(((higher as u16) << 8) | lower as u16),
    ))
}

// マークアンドスペース
fn mark_and_space_hexadecimal(input: &str) -> IResult<&str, MarkAndSpaceIrCarrier> {
    let (input, (m, s)) = tuple((four_digits_hexadecimal, four_digits_hexadecimal))(input)?;
    Ok((input, MarkAndSpaceIrCarrier { mark: m, space: s }))
}

// マークアンドスペースのベクタ
fn take_mark_and_spaces_ircarrier(input: &str) -> IResult<&str, Vec<MarkAndSpaceIrCarrier>> {
    many1(delimited(
        multispace0,
        mark_and_space_hexadecimal,
        multispace0,
    ))(input)
}

// 入力文字列を解析してマークアンドスペースのベクタにする
pub fn from_infrared_code(input: &str) -> Result<Vec<MarkAndSpaceIrCarrier>, String> {
    match take_mark_and_spaces_ircarrier(input).finish() {
        Ok((_, res)) => Ok(res),
        Err(e) => Err(e.to_string()),
    }
}

#[test]
fn test_four_digits_hexadecimal() {
    assert_eq!(
        four_digits_hexadecimal("5601"),
        Ok(("", IrCarrierCounter(0x0156)))
    );
    assert_eq!(
        mark_and_space_hexadecimal("5601AA00"),
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
        take_mark_and_spaces_ircarrier("5601AA00 17001500"),
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

// 数字
fn take_microseconds(input: &str) -> IResult<&str, Microseconds> {
    map_res(take_while(|c: char| c.is_ascii_digit()), |x| {
        u32::from_str(x).map(Microseconds)
    })(input)
}

// マークアンドスペース
fn take_mark_and_space_micros(input: &str) -> IResult<&str, MarkAndSpaceMicros> {
    let (input, mark) = delimited(multispace0, take_microseconds, multispace0)(input)?;
    let (input, _) = char(',')(input)?;
    let (input, space) = delimited(multispace0, take_microseconds, multispace0)(input)?;
    let (input, _) = opt(char(','))(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, MarkAndSpaceMicros { mark, space }))
}

// 入力文字列を解析してマークアンドスペースのベクタにする
pub fn from_array(input: &str) -> Result<Vec<MarkAndSpaceMicros>, String> {
    let mut parse = {
        delimited(char('{'), many1(take_mark_and_space_micros), char('}'))};
    match parse(input).finish() {
        Ok((_, res)) => Ok(res),
        Err(e) => Err(e.to_string()),
    }
}

#[test]
fn test1_from_array() {
    assert_eq!(
        from_array("{1,2}"),
        Ok(vec!(MarkAndSpaceMicros {
            mark: Microseconds(1),
            space: Microseconds(2)
        }))
    );
}

#[test]
fn test2_from_array() {
    assert_eq!(
        from_array("{1,2,}"),
        Ok(vec!(MarkAndSpaceMicros {
            mark: Microseconds(1),
            space: Microseconds(2)
        }))
    );
}

#[test]
fn test3_from_array() {
    assert_eq!(
        from_array("{  1 , 2 , 3 , 4 }"),
        Ok(vec!(
            MarkAndSpaceMicros {
                mark: Microseconds(1),
                space: Microseconds(2)
            },
            MarkAndSpaceMicros {
                mark: Microseconds(3),
                space: Microseconds(4)
            }
        ))
    );
}
