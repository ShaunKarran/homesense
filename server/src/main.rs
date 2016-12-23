use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;


fn handle_client(client_id: u8, mut stream: TcpStream) {
    let mut stream_buffer: [u8; 8] = [0; 8];

    stream.read(&mut stream_buffer).unwrap();
    println!("From client {}: {}", client_id, str::from_utf8(&stream_buffer).unwrap());
}

fn main() {
    let mut client_id: u8 = 0;
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();

    // Accept connections and process them, spawning a new thread for each one.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(client_id, stream));
                client_id += 1;
            }
            Err(e) => {
                println!("{:?}", e); /* Connection failed. Print the error and continue. */
            }
        }
    }
}
