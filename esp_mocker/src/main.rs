extern crate rand;

use std::io::BufWriter;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

use rand::distributions::{IndependentSample, Range};

fn main() {
    // Attempt to connect to a host and panic if failed.
    let mut writer = BufWriter::new(TcpStream::connect("192.168.1.100:12345").unwrap());

    loop {
        let mut rng = rand::thread_rng();
        let number_between = Range::new(15, 30);
        let temperature = number_between.ind_sample(&mut rng).to_string();

        let json = format!("{{\"device_id\": 100, \"temperature\": {}}}\n", temperature);

        writer.write(json.as_bytes()).unwrap();
        match writer.flush() {
            Ok(_) => {
                println!("Wrote: {}", json);
            }
            Err(e) => {
                println!("{:?}", e); /* Write failed. Print the error and continue. */
            }
        }

        sleep(Duration::from_secs(10));
    }
}
