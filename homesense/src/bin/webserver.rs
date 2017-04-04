#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate diesel;
extern crate rocket;

extern crate homesense;

use std::fmt::Write;

use chrono::Local;
use diesel::expression::helper_types::Desc;
use diesel::prelude::*;

use homesense::models::Reading;
use homesense::utils;


#[get("/latest_temp")]
fn latest_temp() -> String {
    use homesense::schema::readings::dsl::*;

    let db_connection = utils::establish_db_connection();

    let results = readings
        // .filter(recorded_at.is_not_null())
        // .filter(recorded_at.lt(Local::now())
        // .filter(device_id.lt(10))
        .order(Desc::new(homesense::schema::readings::recorded_at))
        .limit(5)
        .load::<Reading>(&db_connection)
        .expect("Error loading readings");

    let mut results_string = String::new();

    writeln!(&mut results_string, "<table border='1'>").unwrap();
    writeln!(&mut results_string,
             "<tr><th>Recorded at</th><th>Temperature</th></tr>")
        .unwrap();
    for result in results {
        writeln!(&mut results_string,
                 "<tr><td>{}</td><td>{}C</td></tr>",
                 result.recorded_at.unwrap(),
                 result.temperature.unwrap())
            .unwrap();
    }
    writeln!(&mut results_string, "</table>").unwrap();

    results_string
}

fn main() {
    rocket::ignite().mount("/", routes![latest_temp]).launch();
}
