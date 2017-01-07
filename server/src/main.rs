#![feature(proc_macro)]

extern crate chrono;
extern crate rustc_serialize;

use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

use chrono::{Local, NaiveDateTime};
use rustc_serialize::json;

// These are for diesel. Probably move all this stuff once its working.
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{NewReading};

pub mod schema;
pub mod models;


#[derive(Debug)] // Make printable.
#[derive(RustcDecodable, RustcEncodable)] // Allows decoding of the sensor data.
pub struct SensorData {
    device_id: i32,
    temperature: f32,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// NOTE The deep sleep design for the esp means thats every time it wakes it up it connects again
// and is handled as a new client. This function should probably be changed to not loop.
fn handle_client(stream: TcpStream) {
    let db_connection = establish_connection();
    let mut reader = BufReader::new(stream);
    let mut sensor_data: SensorData;

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        // Attempt to decode the recieved data.
        match json::decode(&line) {
            Ok(decoded) => {
                sensor_data = decoded;
            }
            Err(_) => {
                return;
            }
        }

        // NOTE Since the values might be None when a sensor doesnt exist
        // maybe they will need to be Option type?
        let new_reading = NewReading {
            recorded_at: Local::now().naive_local(),
            device: sensor_data.device_id,
            temperature: sensor_data.temperature,
            humidity: 0.0,
            light: 0.0,
        };

        // Add data to the database.
        use schema::readings;
        diesel::insert(&new_reading)
            .into(readings::table)
            .execute(&db_connection)
            .expect("Error saving new post");

        println!("{:?}", new_reading);
    }
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
