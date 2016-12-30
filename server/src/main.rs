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
use self::models::{NewTemperature};

pub mod schema;
pub mod models;


#[derive(Debug)] // Make printable.
#[derive(RustcDecodable, RustcEncodable)] // Allows decoding of the sensor data.
pub struct SensorData {
    device_id: u8,
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
    let mut reader = BufReader::with_capacity(8, stream);
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

        let local_time = Local::now();
        let local_time_string = local_time.format("%d-%m-%Y %H:%M:%S").to_string();

        // Add data to the database.
        let format = "%d-%m-%Y %H:%M:%S";
        let new_temperature = NewTemperature {
            recorded_at: NaiveDateTime::parse_from_str(&local_time_string, &format).unwrap(),
            temperature: sensor_data.temperature,
        };

        use schema::temperatures;

        diesel::insert(&new_temperature).into(temperatures::table)
            .execute(&db_connection)
            .expect("Error saving new post");

        println!("{}: {:?}", local_time_string, sensor_data);
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
