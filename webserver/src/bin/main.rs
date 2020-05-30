use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5876").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, file_name) = if buffer.starts_with(get){

        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let content = fs::read_to_string(file_name).unwrap();
    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
