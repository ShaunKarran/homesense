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
    // NOTE `recorded_at` is only an Option because I currently decode the JSON data from the esp
    // directly into a `NewReading`. That JSON data does not contain `recorded_at` which is then
    // added by the server. In short this means that `recorded_at` must be able to be None.
    // Unfortunately that also means that the `recorded_at` field in the database by be able to be
    // NULL, which is not ideal as every reading should contain a `recorded_at` value.
    pub recorded_at: Option<NaiveDateTime>,
    pub device_id: i32,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub light: Option<f32>,
}
