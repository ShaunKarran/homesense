extern crate rand;

use std::io::BufWriter;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

use rand::distributions::{IndependentSample, Range};

fn main() {
    let mut writer;

    // Attempt to connect to a host and `panic!`` if fail.
    match TcpStream::connect("127.0.0.1:12345") {
        Ok(stream) => {
            writer = BufWriter::new(stream);
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }

    loop {
        let mut rng = rand::thread_rng();
        let number_between = Range::new(15, 30);
        let temperature = number_between.ind_sample(&mut rng).to_string();

        let _ = writer.write_fmt(format_args!("{}\n", temperature));
        match writer.flush() {
            Ok(_) => {
                println!("Temperature: {}", temperature);
            }
            Err(e) => {
                println!("{:?}", e); /* Write failed. Print the error and continue. */
            }
        }

        sleep(Duration::from_secs(5));
    }
}
