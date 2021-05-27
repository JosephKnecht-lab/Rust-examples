use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();  //Returns Result<T, E>

    for stream in listener.incoming() {
        let stream = stream umwarap();

        println("Connection has been established!");
    }
}
