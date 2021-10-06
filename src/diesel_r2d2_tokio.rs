extern crate r2d2;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

use dotenv::dotenv;
use std::env;

use crate::models::{NewPost, Post};

pub fn runtime() {
    tokio::runtime::Builder::new_current_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
        .block_on(run())
}

pub async fn run() {
    dotenv().ok();

    let pool = create_pool();

    tokio::join!(
        create_post(&pool, "title_1", "body_1"),
        create_post(&pool, "title_2", "body_2"),
        create_post(&pool, "title_3", "body_3"),
        create_post(&pool, "title_4", "body_4")
    );

    let results = get_posts(&pool).await;
    for result in results {
        println!("{:?}", result);
    }
}

pub fn create_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let connection_string = env::var("DATABASE_URL").expect("");
    let manager = ConnectionManager::<PgConnection>::new(connection_string);
    r2d2::Pool::builder().max_size(10).build(manager).unwrap()
}

pub async fn get_posts(pool: &r2d2::Pool<ConnectionManager<PgConnection>>) -> Vec<Post> {
    use crate::schema::posts::dsl::*;

    let db_coon = pool.get().unwrap();

    posts
        .filter(published.eq(true))
        .limit(5)
        .load(&db_coon)
        .expect("Error loading posts")
}

pub async fn create_post<'a>(
    pool: &r2d2::Pool<ConnectionManager<PgConnection>>,
    title: &'a str,
    body: &'a str,
) -> Post {
    use crate::schema::posts;

    let db_coon = pool.get().unwrap();

    let new_post = NewPost { body, title };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&db_coon)
        .expect("Error saving new post")
}
