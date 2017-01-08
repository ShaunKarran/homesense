// NOTE Had trouble getting DateTime (with timezone) working so using NaiveDateTime for now.
use chrono::{NaiveDateTime};
use super::schema::readings;

#[derive(Queryable)]
pub struct Reading {
    pub id: i32,
    pub recorded_at: NaiveDateTime,
    pub device_id: i32,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub light: Option<f32>,
}


#[derive(Debug, Insertable, RustcDecodable)]
#[table_name="readings"]
pub struct NewReading {
    pub recorded_at: Option<NaiveDateTime>,
    pub device_id: i32,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub light: Option<f32>,
}
