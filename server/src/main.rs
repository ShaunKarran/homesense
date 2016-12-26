extern crate chrono;
extern crate rustc_serialize;

use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

use chrono::Local;
use rustc_serialize::json;


#[derive(Debug)] // Make printable.
#[derive(RustcDecodable, RustcEncodable)] // Allows decoding of the sensor data.
pub struct SensorData {
    device_id: u8,
    temperature: f32,
}


fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::with_capacity(8, stream);
    let mut sensor_data: SensorData;

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

        // Attempt to decode the recieved data.
        // If it fails print the error and continue to waiting for the next message.
        match json::decode(&line) {
            Ok(decoded) => {
                sensor_data = decoded;
            }
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        }

        let local_time = Local::now();
        println!("{}: {:?}", local_time.format("%d-%m-%Y %H:%M:%S").to_string(), sensor_data);
    }
}

fn main() {
    let listener = TcpListener::bind("192.168.1.12:12345").unwrap();

    // Accept connections and process them, spawning a new thread for each one.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| handle_client(stream));
            }
            Err(e) => {
                println!("{:?}", e); /* Connection failed. Print the error and continue. */
            }
        }
    }
}
