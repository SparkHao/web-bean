use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, fs, fmt::format};



fn main() {
    println!("Hello, world!");
    let listen = TcpListener::bind("localhost:9999").unwrap();

    for stream in listen.incoming() {
        let stream = stream.unwrap();
        println!("connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("/home/free/code/web-bean/html/index.html").unwrap();

    let response = format!("HTTP/1.1 200 OK \r\nContent-Length: {}\r\n\n\r\n{}", contents.len(), contents);

    println!("request: {}", String::from_utf8_lossy(&buffer[..]));
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();

//     let response = "HTTP/1.1 200 OK \r\n\r\n";

//     println!("request: {}", String::from_utf8_lossy(&buffer[..]));
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap()
// }