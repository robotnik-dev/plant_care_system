mod api;
mod config;
mod db;
mod services;

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use shared::models::reading::Reading;

#[get("/readings")]
fn index() -> Json<Vec<Reading>> {
    let readings = vec![Reading::new(0, 42.0), Reading::new(1, 69.0)];
    Json(readings)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
