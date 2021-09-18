#[macro_use]
extern crate diesel;

mod diesel_get_started;
mod models;
mod schema;

fn main() {
    diesel_get_started::run();
}
