extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate rustc_serialize;

extern crate homesense;

use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

use chrono::Local;
use diesel::prelude::*;
use rustc_serialize::json;

use homesense::models::NewReading;
use homesense::schema::readings;
use homesense::utils;


fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    // Read the contents from the tcp stream into a String.
    reader.read_line(&mut line).expect("Error reading line from BufReader.");

    // Decode the data into a NewReading and record the current date & time.
    let mut new_reading: NewReading = json::decode(&line).expect("Error decoding json data.");
    new_reading.recorded_at = Some(Local::now().naive_local());

    // Add the NewReading to the database.
    let db_connection = utils::establish_db_connection();
    diesel::insert(&new_reading)
        .into(readings::table)
        .execute(&db_connection)
        .expect("Error saving new reading to database.");

    println!("{:?}", new_reading);
}

fn main() {
    let listener = TcpListener::bind("192.168.1.5:12345").unwrap();

    // Accept connections and process them, spawning a new thread for each one.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("{:?}", e); /* Connection failed. Print the error and continue. */
            }
        }
    }
}
