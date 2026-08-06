# mini-http-server

## 目的
HTTPライブラリを使わずにHTTPサーバを作る。
HTTP/1.1の主要機能を入れることを目標とする。
使用言語：Rust

## 機能
- [x] TCP server を作成
- [x] TCP connections を受理
- [x] HTTPリクエストを受信
- [x] HTTP request line (Method / Path / Version) をパースする
- [×] HTTP レスポンスを返す
- [ ] 認証を入れる
- [ ] キャッシュを実装