/*
 * RustのHttpServer
 * CreatedAt: 2019-07-09
 */
use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("接続が確立しました！");
    }
}
