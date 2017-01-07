// NOTE Had trouble getting DateTime (with timezone) working so using NaiveDateTime for now.
// Must include "chrono" as a diesel feature in `Cargo.toml`.
use chrono::{NaiveDateTime};
use super::schema::readings;

#[derive(Queryable)]
pub struct Reading {
    pub id: i32,
    pub recorded_at: NaiveDateTime,
    pub device: i32,
    pub temperature: f32,
    pub humidity: f32,
    pub light: f32,
}

#[derive(Debug, Insertable)]
#[table_name="readings"]
pub struct NewReading {
    pub recorded_at: NaiveDateTime,
    pub device: i32,
    pub temperature: f32,
    pub humidity: f32,
    pub light: f32,
}
