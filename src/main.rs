use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream}; // TcpListener・TcpStream：Rust標準ライブラリにある構造体

fn handle_client(mut stream: TcpStream) {
    // クライアントから送られてきたデータを読み込むためのバッファ
    let mut buffer = [0; 1024];

    // TcpStreamから最大1024バイト読み込む
    let n = stream // n：何バイト読めたか(readの返り値)
        .read(&mut buffer)
        .expect("Failed to read from stream");

    // HTTPはネットワーク上では文字ではなくバイト列として送られてくるので、文字列に変換
    let request = String::from_utf8_lossy(&buffer[..n]);

    // （目標）最初の1行を取り出す
    // 【lines()】
    // GET / HTTP/1.1 Host: ... Connection: ... といった文字列を
    // 行ごとに ["GET / HTTP/1.1", "Host: ...", "Connection: ...", ...] 配列へ変換する
    // 【next()】 最初の１行 = "GET / HTTP/1.1" を取ってくる
    // 【unwrap()】 Some(引数)の引数のみを取り出す
    let request_line = request.lines().next().unwrap();

    // "GET / HTTP/1.1" を空白で区切って、配列にする
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    // 配列のパターンマッチで分解して、表示
    if let [method, path, version] = parts.as_slice() {
        // method, path, version の中身はポインタであることに注意！

        // method をHTTPメソッド別の処理を書く
        match *method {
            "GET" => {
                println!("=== GET ===");
                println!("Retrieve resource");

                // ヘッダとボディ
                let response = "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/plain\r\n\
                    Content-Length: 13\r\n\
                    \r\n\
                    Hello, World!";

                // streamに渡す
                stream
                    .write_all(response.as_bytes()) // 8ビット符号なし整数（unsigned 8-bit integer）
                    .expect("Failed to write response")
            }
            "POST" => {
                println!("=== POST ===");
                println!("Create new resource");
            }
            "PUT" => {
                println!("=== PUT ===");
                println!("Update resource");
            }
            "DELETE" => {
                println!("=== DELETE ===");
                println!("Delete resource");
            }
            _ => {
                println!("Unknown method");
            }
        }
        println!("Method : {}", method);
        println!("Path   : {}", path);
        println!("Version: {}", version);
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080") // bind：「自分のPCの8080番アドレスを使います」とOSに登録する
        .expect("Failed to bind"); // 失敗したら、引数のメッセージを出して終了
    // listener：待ち受け準備ができたサーバ本体

    println!("Server is running at http://127.0.0.1:8080");

    // 接続が来るたびに TcpStream を受け取る
    for stream in listener.incoming() {
        // listener.incoming()：OSの待ち受けキュー
        match stream {
            Ok(mut stream) => {
                println!("Client connected!");
                handle_client(stream)
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}
