mod api;
mod config;
mod db;
mod services;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
