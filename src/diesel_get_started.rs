use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::NewPost;
use crate::models::Post;

pub fn run() {
    let db_connection = establish_connection();

    let post_created = create_post(&db_connection, "First Title", "First Body");
    update_post(&db_connection, post_created.id);

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

pub fn get_posts(db_conn: &PgConnection) -> Vec<Post> {
    use crate::schema::posts::dsl::*;

    posts
        .filter(published.eq(true))
        .limit(5)
        .load(db_conn)
        .expect("Error loading posts")
}

pub fn create_post<'a>(db_coon: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { body, title };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(db_coon)
        .expect("Error saving new post")
}

pub fn update_post(db_conn: &PgConnection, id: i32) -> Post {
    use crate::schema::posts::dsl::{posts, published};

    diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result(db_conn)
        .expect(&format!("Unable to find post {}", id))
}
