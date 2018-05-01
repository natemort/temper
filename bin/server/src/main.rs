#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate temper_common;
extern crate influx_db_client;

use influx_db_client::{Client, Point, Value, Precision};
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::{Json};
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};
use temper_common::protocol::{TemperatureReading, Response};

struct InfluxProvider {
    host: String,
    db: String,
}

impl InfluxProvider {
    fn new(host: &'static str,  db: &'static str) -> InfluxProvider {
        InfluxProvider{host: host.to_string(), db: db.to_string()}
    }
    fn with_connection<R, E: Error, T: FnOnce(&Client)->Result<R, E>>(&self, function: T) -> Result<R, E>
    {
        let client = Client::new(&self.host, &self.db);
        return function(&client);
    }
}

#[put("/<id>", format="application/json", data="<reading>")]
fn add_reading(id: u64, reading: Json<TemperatureReading>, influx: State<InfluxProvider>) -> Json<Response> {
    let reading = reading.into_inner();
    let point = Point::new("reading")
        .add_tag("device", Value::String(id.to_string()))
        .add_field("temperature", Value::Float(reading.temperature.into()))
        .add_field("humidity", Value::Float(reading.humidity.into()))
        .to_owned();
    let result = influx.with_connection(|con| {
        con.write_point(point, Some(Precision::Seconds), None)
    });
    return Json(match result {
        Ok(_) => Response::Success,
        Err(e) => Response::Error {message: e.description().to_string()}
    });
}

#[get("/static/<file..>")]
fn get_static(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("bin/server/static/").join(file)).ok()
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("bin/server/static/index.html")
}

fn main() {
    rocket::ignite()
        .manage(InfluxProvider::new("localhost:8086", "temper"))
        .mount("/", routes![index, get_static])
        .mount("/api/readings", routes![add_reading])
        .launch();
}