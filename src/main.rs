#[macro_use]
extern crate diesel;

mod diesel_get_started;
mod diesel_r2d2;
mod diesel_r2d2_tokio;
mod mock_all;
mod models;
mod schema;

fn main() {
    diesel_get_started::run();
}
