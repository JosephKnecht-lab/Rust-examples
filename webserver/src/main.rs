use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();  //Returns Result<T, E>

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection has been established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();  

    let get = b"GET / HTTP/1.1\r\n";  //b is for byte string

    if buffer.starts_with(get){ //handle GET
        let mut file = File::open("index.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        //println!("Request: {}",String::from_utf8_lossy(&buffer[..]));
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    } else { //Handle 404
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        
        let mut file = File::open("404.html").unwrap(); 
        let mut contents = String::new(); 
        
        file.read_to_string(&mut contents).unwrap();
        
        let response = format!("{}{}", status_line, contents); 
        
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    }
}