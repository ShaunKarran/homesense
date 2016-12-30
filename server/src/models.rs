// NOTE Had trouble getting DateTime (with timezone) working so using NaiveDateTime for now.
// Must include "chrono" as a diesel feature in `Cargo.toml`.
use chrono::{NaiveDateTime};
use super::schema::temperatures;

#[derive(Queryable)]
pub struct Temperature {
    pub id: i32,
    pub recorded_at: NaiveDateTime,
    pub temperature: f32
}

#[derive(Insertable)]
#[table_name="temperatures"]
pub struct NewTemperature {
    pub recorded_at: NaiveDateTime,
    pub temperature: f32
}
