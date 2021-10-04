use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::NewPost;
use crate::models::Post;

pub fn run() {
    dotenv().ok();
    let db_connection = establish_connection();

    let post_created = create_post(&db_connection, "First Title", "First Body");
    update_post(&db_connection, post_created.id);

    let results = get_posts(&db_connection);

    for post in results {
        println!("{:?}", post);
        delete_post(&db_connection, post.title);
    }
}

pub fn establish_connection() -> PgConnection {
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

pub fn delete_post(db_conn: &PgConnection, target: String) {
    use crate::schema::posts::dsl::*;

    let pattern = format!("%{}%", target);

    diesel::delete(posts.filter(title.like(pattern)))
        .execute(db_conn)
        .expect("Error deleting post");
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::result::Error;
    use dotenv;

    fn config_env() {
        dotenv::from_filename(".env").ok();
    }

    #[test]
    fn should_establish_connection() {
        config_env();
        let db_coon = establish_connection();

        db_coon.test_transaction::<_, Error, _>(|| {
            let post_to_create = Post {
                id: 0,
                body: String::from("Some Body"),
                title: String::from("Some Title"),
                published: false,
            };
            let post_created = create_post(
                &db_coon,
                post_to_create.title.as_str(),
                post_to_create.body.as_str(),
            );
            assert_eq!(post_created.title, post_to_create.title);
            assert_eq!(post_created.body, post_to_create.body);
            assert_eq!(post_created.published, post_to_create.published);

            let post_updated = update_post(&db_coon, post_created.id);
            assert_eq!(post_updated.published, true);

            let all_posts = get_posts(&db_coon);
            assert_eq!(all_posts[0].title, post_to_create.title);
            assert_eq!(all_posts[0].body, post_to_create.body);

            Ok(())
        })
    }
}
