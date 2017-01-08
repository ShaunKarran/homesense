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

pub mod schema;
pub mod models;

#[derive(Debug)] // Make printable.
#[derive(RustcDecodable, RustcEncodable)] // Allows decoding of the sensor data.
pub struct SensorData {
    device_id: i32,
    temperature: f32,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn decode_sensor_data(data_string: &String) -> NewReading {
    // Attempt to decode the recieved data.
    // NOTE Handle this unwrap properly.
    let sensor_data: SensorData = json::decode(data_string).unwrap();

    // NOTE Since the values might be None when a sensor doesnt exist
    // maybe they will need to be Option type?
    NewReading {
        recorded_at: Local::now().naive_local(),
        device: sensor_data.device_id,
        temperature: sensor_data.temperature,
        humidity: 0.0,
        light: 0.0,
    }
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();
    let new_reading = decode_sensor_data(&line);

    // Add data to the database.
    let db_connection = establish_connection();
    use schema::readings;
    diesel::insert(&new_reading)
        .into(readings::table)
        .execute(&db_connection)
        .expect("Error saving new post");

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
