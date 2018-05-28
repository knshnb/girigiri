# Girigiri
Shogi AI written in Rust

[日本語バージョン](README.jp.md)

## Install

1. [Install Rust](https://www.rust-lang.org/en-US/install.html)
2. Clone

    ```
    $ git clone https://github.com/bknshn/girigiri
    ```

## How to play
1. Build

    ```
    $ cargo build --release
    ```
2. Install [将棋所(Shogidokoro)](http://www.geocities.jp/shogidokoro/index.html)
3. Open Shogidokoro.exe and set girigiri/target/release/usi as engine
- In Linux or Mac you need to use [Mono](https://www.mono-project.com/docs/getting-started/install/)
```
mono /usr/local/bin/Shogidokoro/Shogidokoro.exe
```

## Others
- Run self match locally

    ```
    $ cargo run --bin auto --release
    ```

- Battle in [floodgate](http://wdoor.c.u-tokyo.ac.jp/shogi/floodgate.html)

    ```
    $ cargo run --bin floodgate --release
    ```

- For [CSA](http://www.computer-shogi.org/protocol/tcp_ip_server_121.html) battle

    ```
    $ cargo run --bin main --release
    ```

- Learn evaluation function

    ```
    $ cargo run --bin learn --release
    ```

- Debug

    ```
    $ cargo run --bin debug --release
    ```

## Introduction
[Rustで将棋AI入門 1-動かしてみる](https://qiita.com/bknshn/items/a989f223aa022c0c9c2a)
