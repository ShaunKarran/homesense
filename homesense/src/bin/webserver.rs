#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate rocket;

extern crate homesense;

use diesel::expression::helper_types::Desc;
use diesel::prelude::*;

use homesense::models::Reading;
use homesense::schema::readings::dsl::readings;
use homesense::utils;


#[get("/current_temp")]
fn current_temp() -> String {
    let db_connection = utils::establish_db_connection();

    // Use descending order and limit of 1 to get latest reading.
    let results = readings
        .order(Desc::new(homesense::schema::readings::recorded_at))
        .limit(1)
        .load::<Reading>(&db_connection)
        .expect("Error loading readings");

    format!("The current temperature is {}C", results[0].temperature.unwrap())
}

fn main() {
    rocket::ignite().mount("/", routes![current_temp]).launch();
}
