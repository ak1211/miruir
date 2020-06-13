# miruir
PiHVAC https://github.com/ak1211/pi_hvac/
のリモコン信号解析エンジンのみを取り出してUIをReact Native WindowsにしてVisual StudioでUWPアプリにしたもの。

# UWPアプリ
releaseタブのv0.1 Assets/miruir_0.1.2.0_x86_x64_arm.appxbundle
開発者モードのWindowsでインストール。

証明書ではじかれるかもしれないので、そのときはソースからビルドして。

# ソースからのビルド
## React Native Windowsのインストール
https://github.com/microsoft/react-native-windows
を参照してReact Native Windowsをインストールする。

Visual Studio 2019が入ったら
お好みでvscodeを入れる。

## PureScriptのインストール
すでにnodejsがあるはずなので
https://www.purescript.org/
を参照してPureScriptをインストールする。

## spagoのインストール
PureScriptに続いてspagoをインストールする。

## ビルド
管理者権限のPowerShellで行わないとよくわからないエラーがでる。

### .pursファイルをコンパイル
> spago build

### VSCode
> code .
VSCode上でF5。

**または**
./windows/miruir.slnをVisual Studioで開いて実行
