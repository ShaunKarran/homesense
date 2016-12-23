extern crate rand;

use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

use rand::distributions::{IndependentSample, Range};

fn main() {
    let mut stream;

    // Attempt to connect to a host and `panic!`` if fail.
    match TcpStream::connect("127.0.0.1:12345") {
        Ok(tcp_stream) => {
            stream = tcp_stream;
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }

    let mut rng = rand::thread_rng();
    let number_between = Range::new(15, 30);
    let mut temperature;

    loop {
        temperature = number_between.ind_sample(&mut rng);

        match stream.write(temperature.to_string().as_bytes()) {
            Ok(_) => {
                println!("Temperature: {}C", temperature);
            }
            Err(e) => {
                println!("{:?}", e); /* Write failed. Print the error and continue. */
            }
        }

        sleep(Duration::from_secs(60));
    }
}
