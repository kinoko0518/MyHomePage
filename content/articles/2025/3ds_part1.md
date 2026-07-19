+++
title = "Rustで3DSソフトを作る「環境構築」 "
date = 2025-07-13

[extra]
cover = "images/covers/rust_3ds_dev.webp"
abstract = "Rustで3DS向けの開発を行うためのチュートリアルです。もともとC言語向けのライブラリであるdevkitProのRust用ラッパーを使っています。"
+++

さて、いきなりだが、3DSは好きだろうか。私は大好きだ。パカッと開ければすぐ遊べる手軽さ、3Dスクリーン（誰も使っていないとは何事か？）、そしてポケットに収まるコンパクトさ。最高だ。

して、平成のころに子供で、ゲームクリエイターにぼんやりとした憧れを持ちながらScratchを遊んでいた人は一度は思ったことがあるのではないか？

「3DSで自作ゲームを動かしたいなあ」

今日はそんないたいけなクソガキだったころの私の夢を叶えてあげようと思う。さて、早速本題に入ろう。環境はLinux、Arch系を想定する。

# はじめに

本来、3DSの仕様書は任天堂と契約したゲーム会社だけがアクセスでき、一般開発者にはアクセスできないようになっている。

しかし3DSといえど所詮はコンピュータ、ギークたちの執念の前には勝ち目はない。そんなギークたちの執念の集大成がdevkitProだ。devkitProは3DSのレジスタをマッピングし、低レベルな制御をラッピングし3DS向けのビルドを行う。

しかし一つ問題がある。このライブラリ、C用なのである。私は誇り高き？Rustacean（著者注・Rustの熱心なユーザ）。ここでCに屈すればCrustacean、ただのカニへと成り果ててしまう。私はFirefoxの背中に飛び乗りGitHubの大海へと漕ぎ出した。

## そもそもの3DSの話

3DSには当然ながら無改造で自作アプリをインストールすることはできない。ここで施す改造がCFW(Custom Firmware)だ。しかし、CFWの導入に関しては解説してくださっている方が沢山いるため、ここでは省くこととする。以下の説明はすべてCFW導入環境を前提としている。以下に説明を理解するための頻出単語の解説を行う。

### Homebrew
自家醸造という意味の英単語で、任天堂公式ではないユーザーが作成したソフトウェアのことを指す。Homebrewを起動するためのソフトであるHomebrew Launcherでは以下のように説明されている。

> What is homebrew ?
>
> Homebrew is what we call unofficial software made by amateur developers for closed systems such as the 3DS. This includes both games and applications, and in practice getting homebrew on your 3DS means you'll be able to :
>
> - Play Aperture Science 3D, a free adaptation of Portal for the 3DS.
> - Play out-of-region games you own.
> - Make your own themes to use in home menu.
> - Play old SNES games with blargSNES.

> Homebrewって何やねん(著者意訳)
>
> Homebrewはアマチュア開発者が3DSのような閉じられたシステム向けに作成した非公式なソフトウェアのことやで。これはゲームとアプリの両方を含んどってな、たとえば3DSだとHomebrewを使ってができるようになんねん:
>
> - Aperture Science 3D（Portalの3DS向けの無料の移植）をプレイする
> - リージョンロックされたゲームを遊ぶ
> - 自作のホームテーマを適用する
> - blargSNESを使ってスーファミのゲームを遊ぶ

### 3dsx、cia

3dsx(3DS Executable)は先述のHomebrew Launcherで起動できる実行ファイルだ。欠点としてはホーム画面から起動できず、一度Homebrew Launcherを経由しないといけないというものがある。まずこれが不便なのと、自作ゲームがホーム画面から隔離されているのはテンションが上がらない。ロマンがない。

そこで登場するのがciaだ。こいつは簡単に言えばインストーラー、eShopからインストールしてきたかのようにネイティブにインストールし、ホーム画面にアイコンを表示してくれる。そして3dsxは有志の方が配布しているスクリプトを使えばciaに変換可能だ。私はロマンを感じずにはいられない。

### ctru-rsとcargo-3ds

ctru-rsは3DSの低レベルな機能やOS機能へのアクセスを行うライブラリ、libctruのRust用ラッパー、cargo-3dsは3dsxをビルドする作業をいつも通りcargoでビルドするように1コマンドで出来るようにしてくれるソフトだ。

しかし、である。これらはdevkitProといった外部アプリケーションに依存する。そして著者はここで泥濘にはまってしまった。今回は環境構築編ということで、読者の皆様がこの泥濘をスマートに越える一助となるよう手順を示そうと思う。

# 環境構築

さて、まずはctru-rsの依存関係から解決しよう。ctru-rsはdevkitProの一部であるlibctruのラッパーだ。devkitProをインストールするのが道理というものだろう。

## 環境変数の設定

インストールに際して、まず環境変数を設定する。まだインストールもしていないのにパスを通すというのはなんだか変な気もするが、devkitProはここで設定したパスにインストールを行う。実質的にこの手順がインストールパスの設定というわけだ。

/etc/profile.d/はシェルが起動時に中のシェルスクリプトを自動的にすべて実行する特殊なディレクトリだ。Arch Wikiにはこのように記載がある。

> /etc/profile は、ログインシェルのみに対して変数を初期化します。しかし、このファイルはスクリプト (例: /etc/profile.d/ 内にあるファイル) を実行し、すべての Bourne shell 互換シェルで使用することができます。

ここにセッション内の環境変数の設定を行うシェルスクリプトを登録することによって「環境変数を登録した」ということになるわけである。早速シェルスクリプトを作成していこう。vimやmicroなどのお好みのテキストエディタを用意してほしい。本記事では特に説明がない限りmicroでの例を示すが、どのエディタを使ってもらっても構わない。

```
$ sudo touch /etc/profile.d/devkitpro.sh
$ micro /etc/profile.d/devkitpro.sh
```

空のファイルが作成されるので以下の内容を貼り付ける。devkitpro.shの名前はお好みで変更しても問題ない。

```
# devkitProの大元のパス
export DEVKITPRO=/opt/devkitpro
# 特殊な環境にコンパイルするためのCコンパイラ
export DEVKITARM=$DEVKITPRO/devkitARM
export DEVKITPPC=$DEVKITPRO/devkitPPC
# arm-none-eabi-gccとかのバイナリをコマンドとして認識させる
export PATH=$PATH:$DEVKITARM/bin
export PATH=$PATH:$DEVKITPRO/tools/bin
```

profile.dは保護されているディレクトリなので書き込みには管理者権限が必要だ。きちんと保存されているかcatなどのコマンドを用いて確認することを推奨する。環境変数の変更を適用するため、以下のコマンドを実行して作成したシェルスクリプトを実行しよう。

```
$ source /etc/profile.d/devkitpro.sh
```

## GPGキーリングのインストール

GPGキーは開発者がそのプログラムを自分が作成し提供したと証明するために使われる。これによりユーザーはそのプログラムが信頼できる開発者の作成したものであることを確認できるのである。GPGキーリングはまさにGPGキーの鍵束(keyring)、ひと纏まりのGPGキーを一気にインストールするためのものだ。今回はこれを使ってdevkitProの開発者たちのGPGキーリングを一気にインストールし、信頼する。

```
$ sudo pacman-key --recv BC26F752D25B92CE272E0F44F7FD5492264BB9D0 --keyserver keyserver.ubuntu.com
$ sudo pacman-key --lsign BC26F752D25B92CE272E0F44F7FD5492264BB9D0
$ sudo pacman -U https://pkg.devkitpro.org/devkitpro-keyring.pkg.tar.zst
```

## pacmanの設定

devkitProは自身の管理をpacman、Archで標準で使われているパッケージマネージャを用いて行う。devkitProは専用のpacmanリポジトリ（パッケージのデータベースのようなもの）を持っているのでこれに問い合わせる形になるわけだ。しかし、まっさらのArchはこのリポジトリを知らない。Archにこのリポジトリを教えるためconfigファイルを書き換えよう。

```
$ micro /etc/pacman.conf
```

以下のようなファイルが開ければ成功だ。

```toml
#
# /etc/pacman.conf
#
# See the pacman.conf(5) manpage for option and repository directives

#
# GENERAL OPTIONS
#
[options]
# The following paths are commented out with their default values listed.
# If you wish to use different paths, uncomment and update the paths.
...(以下にもコンフィグは続く)
```

一番下にコードブロック内のコンフィグを貼り付ける。

```toml
[dkp-libs]
Server = https://pkg.devkitpro.org/packages

[dkp-linux]
Server = https://pkg.devkitpro.org/packages/linux/$arch/
```

ここまで編集したらコンフィグファイルを保存して、仕上げに以下のコマンドを実行しパッケージのカタログを最新に更新し、3DS向けの開発に必要な諸々をインストールする。

```
$ sudo pacman -Syu
$ sudo pacman -S 3ds-dev
```

これでdevkitProの基本パッケージと3DSでの開発に必要な諸々がインストールされたはずだ。次のセクションではこれらの諸々をRustで扱うための解説を行う。

## cargo-3dsのインストール

cargo-3dsはcargoからは独立したソフトウェアだがcargoのサブコマンドとして動作する。インストールを行うコマンドを以下に示す。ただし、rustcやcargoなどはインストール済みであることを前提とする。

```
$ cargo install cargo-3ds
```

次にコンパイルを行うために必要な依存関係をインストールする。一行目でインストールしているのはLLVM。OSやデバイスで異なる機械語の一段階前として生成される中間表現と呼ばれるものを扱うためのソフトウェアである。

二行目でインストールしているのはRustのnightly版、毎晩リリースされるα版のようなものである。cargo-3dsはnightly版でしか動作しないためインストールする。このプロジェクトにのみnightlyが適用されるため環境汚染の心配はない。SDGsである。

```
$ sudo pacman -S llvm
$ rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

次に、作業用の適当なディレクトリに移動してプロジェクトを作成する。cargo initのような通常のcargoの機能で作成されたプロジェクトとは構造が異なるので必ず以下のコマンドを用いること。

```
$ cargo 3ds new (適当な名前)
```

main.rsには既に3DSで動作するための最低限の内容が記述されているので早速ビルドしてみよう。

```
$ cargo +nightly 3ds build
```

おそらくは(プロジェクトのルートディレクトリ)/target/armv6k-nintendo-3ds/debug/に(プロジェクト名).3dsxが生成されているはずだ。さて、これで環境構築は終了である。次回は簡単なfizzbuzzから始まり、画像を表示させてみる予定だ。

再見！
