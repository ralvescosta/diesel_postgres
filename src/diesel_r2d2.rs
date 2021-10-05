extern crate r2d2;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

use dotenv::dotenv;
use std::env;

use crate::models::{NewPost, Post};

pub fn create_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let connection_string = env::var("DATABASE_URL").expect("");
    let manager = ConnectionManager::<PgConnection>::new(connection_string);
    r2d2::Pool::builder().max_size(10).build(manager).unwrap()
}

pub fn run() {
    dotenv().ok();

    let pool = create_pool();

    let post_created = create_post(&pool, "First Title", "First Body");
    update_post(&pool, post_created.id);

    let results = get_posts(&pool);

    for post in results {
        println!("{:?}", post);
        delete_post(&pool, post.title);
    }
}

pub fn get_posts(pool: &r2d2::Pool<ConnectionManager<PgConnection>>) -> Vec<Post> {
    use crate::schema::posts::dsl::*;

    let db_coon = pool.get().unwrap();

    posts
        .filter(published.eq(true))
        .limit(5)
        .load(&db_coon)
        .expect("Error loading posts")
}

pub fn create_post<'a>(
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

pub fn update_post(pool: &r2d2::Pool<ConnectionManager<PgConnection>>, id: i32) -> Post {
    use crate::schema::posts::dsl::{posts, published};

    let db_coon = pool.get().unwrap();

    diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result(&db_coon)
        .expect(&format!("Unable to find post {}", id))
}

pub fn delete_post(pool: &r2d2::Pool<ConnectionManager<PgConnection>>, target: String) {
    use crate::schema::posts::dsl::*;

    let db_coon = pool.get().unwrap();

    let pattern = format!("%{}%", target);

    diesel::delete(posts.filter(title.like(pattern)))
        .execute(&db_coon)
        .expect("Error deleting post");
}
