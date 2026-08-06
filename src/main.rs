mod handler;

use handler::handle_client;
use std::net::TcpListener; // TcpListener・TcpStream：Rust標準ライブラリにある構造体

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080") // bind：「自分のPCの8080番アドレスを使います」とOSに登録する
        .expect("Failed to bind"); // 失敗したら、引数のメッセージを出して終了
    // listener：待ち受け準備ができたサーバ本体

    println!("Server is running at http://127.0.0.1:8080");

    // 接続が来るたびに TcpStream を受け取る
    for stream in listener.incoming() {
        // listener.incoming()：OSの待ち受けキュー
        match stream {
            Ok(stream) => {
                println!("Client connected!");
                // クライアントからのデータを分析し処理する
                handle_client(stream)
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}
