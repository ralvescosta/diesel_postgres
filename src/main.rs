#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::Post;
use std::env;

use crate::models::NewPost;

mod models;
mod schema;

fn main() {
    let db_connection = establish_connection();

    create_post(&db_connection, "First Title", "First Body");

    let results = get_posts(&db_connection);

    for post in results {
        println!("{:?}", post);
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_posts(db_conn: &PgConnection) -> Vec<models::Post> {
    use crate::schema::posts::dsl::*;

    posts
        .filter(published.eq(true))
        .limit(5)
        .load(db_conn)
        .expect("Error loading posts")
}

pub fn create_post<'a>(db_coon: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost { body, title };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(db_coon)
        .expect("Error saving new post")
}
