// Copyright (c) 2022 Akihiro Yamamoto.
// Licensed under the MIT License <https://spdx.org/licenses/MIT.html>
// See LICENSE file in the project root for full license information.
//

// 受信コード
export type RxIrRemoteCode = MarkAndSpace[]

// 送信コード
export type TxIrRemoteCode = InfraredRemoteDemodulatedFrame[]

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
