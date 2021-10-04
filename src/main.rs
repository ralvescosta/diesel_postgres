#[macro_use]
extern crate diesel;

mod diesel_get_started;
mod diesel_get_started_with_test;
mod mock_all;
mod models;
mod schema;

fn main() {
    diesel_get_started::run();
}
