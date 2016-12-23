use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;


fn handle_client(client_id: u8, stream: TcpStream) {
    let mut reader = BufReader::with_capacity(8, stream);

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return;
                }
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
        print!("Client {}: {}", client_id, line);
    }
}

fn main() {
    let mut client_id: u8 = 0;
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();

    // Accept connections and process them, spawning a new thread for each one.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| handle_client(client_id, stream));
                client_id += 1;
            }
            Err(e) => {
                println!("{:?}", e); /* Connection failed. Print the error and continue. */
            }
        }
    }
}
