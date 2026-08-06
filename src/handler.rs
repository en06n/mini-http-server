use std::fs; // index.htmlなどのソースを読み込むためのライブラリ
use std::io::{Read, Write};
use std::net::TcpStream; // TcpListener・TcpStream：Rust標準ライブラリにある構造体

// pub を使えないとmain.rsで使えない
pub fn handle_client(mut stream: TcpStream) {
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

                let html =
                    fs::read_to_string("static/index.html").expect("Failed to read index.html");
                // ヘッダとボディ
                // format!：作った文字列を変数に入れるマクロ（コンパイル時にコードが生成される）
                let response = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/html\r\n\
                    Content-Length: {}\r\n\
                    \r\n\
                    {}",
                    html.len(), // 1つ目の{}に入る
                    html        // 2つ目の{}に入る
                );

                // streamに渡す
                stream
                    .write_all(response.as_bytes()) // 8ビット符号なし整数（unsigned 8-bit integer）
                    .expect("Failed to write response")
            }
            "POST" => {
                println!("=== POST ===");
                let response = "HTTP/1.1 405 Method Not Allowed\r\n\
                    \r\n\
                    ";
                stream
                    .write_all(response.as_bytes())
                    .expect("Failed to write response")
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
