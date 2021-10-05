#[macro_use]
extern crate diesel;

mod diesel_get_started;
mod diesel_get_started_with_test;
mod diesel_r2d2;
mod mock_all;
mod models;
mod schema;

fn main() {
    diesel_get_started::run();
}
