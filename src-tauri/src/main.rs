// Copyright (c) 2022 Akihiro Yamamoto.
// Licensed under the MIT License <https://spdx.org/licenses/MIT.html>
// See LICENSE file in the project root for full license information.
//
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[allow(dead_code)]
mod infrared_remote;
#[allow(dead_code)]
mod parsing;

use crate::infrared_remote::*;
use crate::parsing::*;
use std;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            parse_infrared_code,
            decode,
            encode2,
            encode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn parse_infrared_code(ircode: &str) -> Result<Vec<MarkAndSpaceMicros>, String> {
    fn convert(x: Vec<MarkAndSpaceIrCarrier>) -> Vec<MarkAndSpaceMicros> {
        x.iter().map(|x| MarkAndSpaceMicros::from(*x)).collect()
    }

    from_infrared_code(ircode).map(convert)
}

#[tauri::command]
fn decode(
    input: Vec<MarkAndSpaceMicros>,
) -> Result<Vec<InfraredRemoteDemodulatedFrame>, String> {
    let frames = decode_phase1(&input)?;
    Ok(frames
        .iter()
        .map(|frame| decode_phase2(frame))
        .collect::<Vec<InfraredRemoteDemodulatedFrame>>())
}

#[tauri::command]
fn encode2(input: Vec<InfraredRemoteDemodulatedFrame>) -> Result<Vec<MarkAndSpaceMicros>, String> {
    let frames = input
        .iter()
        .map(|x| encode_phase1(x))
        .collect::<Result<Vec<InfraredRemoteFrame>, String>>()?;
    Ok(encode_phase2(&frames))
}

#[tauri::command]
fn encode(input: Vec<InfraredRemoteDemodulatedFrame>) -> Result<String, String> {
    encode_infrared_remote_code(&input)
}
