// Copyright (c) 2022 Akihiro Yamamoto.
// Licensed under the MIT License <https://spdx.org/licenses/MIT.html>
// See LICENSE file in the project root for full license information.
//
import { invoke } from '@tauri-apps/api/tauri'

// 受信コード
export type RxIrRemoteCode = MarkAndSpace[]

// 送信コード
export type TxIrRemoteCode = InfraredRemoteDemodulatedFrame[]

export type RxTxIrRemoteCode =
	| { RxIrRemoteCode: RxIrRemoteCode }
	| { TxIrRemoteCode: TxIrRemoteCode }

//
export const convert_to_RxIrRemoteCode = (input:RxTxIrRemoteCode): RxIrRemoteCode =>{
	if ("RxIrRemoteCode" in input) {
		return input.RxIrRemoteCode
	} else if ("TxIrRemoteCode" in input) {
		var output:RxIrRemoteCode  =[]
		invoke<RxIrRemoteCode>("encode2", { input: input})
		.then(x => { output = x })
		return output
	} else {
		throw new Error('unimplemented')
	}
}

//
export const convert_to_TxIrRemoteCode = (input: RxTxIrRemoteCode): TxIrRemoteCode => {
	if ("RxIrRemoteCode" in input) {
		var output:TxIrRemoteCode  =[]
		invoke<TxIrRemoteCode>("decode", { input: input})
		.then(x => { output = x })
		return output
	} else if ("TxIrRemoteCode" in input) {
		return input.TxIrRemoteCode
	} else {
		throw new Error('unimplemented')
	}
}

//
// バックエンドとの通信用
//

export interface MarkAndSpace {
	mark: number,
	space: number,
};

export type InfraredRemoteDemodulatedFrame =
	| { Aeha: Uint8Array }
	| { Nec: Uint8Array }
	| { Sirc: Uint8Array }
	| { Unknown: MarkAndSpace[] }
