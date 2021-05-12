use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let mut file = File::open("index.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(), /* contents length 가 헤더에 없으면 브라우저들이 제대로 읽지 못한다. 
                           브라우저마다 결과가 달랐는데, 브라우저는 현제 content의 사이즈가 얼마인지 모르는 상황에서
                           파폭은 계속 데이터를 기다리다가 연결을 끊어버리고(RST),
                           크롬은 계속 대기하고, 사파리는 먼저 받은것부터 출력하는 것으보 보인다.
                           RST로 끊으면 안 되고 FIN/FIN-ACK/ACK으로 끊어야하는데
                           파폭, 크롬 등은 RST로 끊어버리면 있는 데이터를 드랍한다.
                           * RST는 Reset의 약자로 말 그대로 강제로 연결을 끊는 것
                           * Content-Length도 넣어야하고, graceful shutdown도 해줘야한다.
                        */
        contents
    );

    println!("{}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // stream.shutdown(std::net::Shutdown::Both); // RST로 끊으면 데이터를 드롭하니 FIN으로 연결을 끊도록 바꿔주는 것.
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}