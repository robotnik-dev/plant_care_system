mod api;
mod config;
mod db;
mod services;

#[macro_use]
extern crate rocket;

use shared::models::reading::Reading;

#[get("/")]
fn index() -> &'static str {
    let _reading = Reading::new(0, 0.43);
    "Hello, world!"
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
