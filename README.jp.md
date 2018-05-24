# Girigiri
Rustで書かれた将棋AIです。

## 使い方

1. Rustのインストール(https://www.rust-lang.org/en-US/install.html)
2. Girigiriのインストール

    ```
    $ git clone https://github.com/bknshn/girigiri
    ```

3. 実行
- ローカルで自己対戦

    ```
    $ cargo run --bin auto --release
    ```

- [floodgate](http://wdoor.c.u-tokyo.ac.jp/shogi/floodgate.html)での対局

    ```
    $ cargo run --bin floodgate --release
    ```

- [CSA](http://www.computer-shogi.org/protocol/tcp_ip_server_121.html)の大会用

    ```
    $ cargo run --bin main --release
    ```

- 評価関数の学習

    ```
    $ cargo run --bin learn --release
    ```

- デバッグ

    ```
    $ cargo run --bin debug --release
    ```

## 入門
[Rustで将棋AI入門 1-動かしてみる](https://qiita.com/bknshn/items/a989f223aa022c0c9c2a)
