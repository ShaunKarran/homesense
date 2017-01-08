#![feature(proc_macro)]

extern crate chrono;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate rustc_serialize;

use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

use chrono::Local;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rustc_serialize::json;

use self::models::NewReading;
use schema::readings;

pub mod schema;
pub mod models;


fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    // Read the contents from the tcp stream into a String.
    reader.read_line(&mut line).unwrap();

    // Decode the data into a NewReading and record the current date & time.
    let mut new_reading: NewReading = json::decode(&line).unwrap();
    new_reading.recorded_at = Some(Local::now().naive_local());

    // Add the NewReading to the database.
    let db_connection = establish_connection();
    diesel::insert(&new_reading)
        .into(readings::table)
        .execute(&db_connection)
        .expect("Error saving new reading.");

    println!("{:?}", new_reading);
}

fn main() {
    let listener = TcpListener::bind("192.168.1.100:12345").unwrap();

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
