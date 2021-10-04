#[cfg(test)]
mod tests {
    use diesel::prelude::*;
    use diesel::result::Error;
    use dotenv;

    use crate::diesel_get_started::*;
    use crate::models::Post;

    fn config_env() {
        dotenv::from_filename(".env").ok();
    }

    #[test]
    fn should_establish_connection() {
        config_env();
        let db_conn = establish_connection();

        db_conn.test_transaction::<_, Error, _>(|| {
            let post_to_create = Post {
                id: 0,
                body: String::from("Some Body"),
                title: String::from("Some Title"),
                published: false,
            };
            let post_created = create_post(
                &db_conn,
                post_to_create.title.as_str(),
                post_to_create.body.as_str(),
            );
            assert_eq!(post_created.title, post_to_create.title);
            assert_eq!(post_created.body, post_to_create.body);
            assert_eq!(post_created.published, post_to_create.published);

            let post_updated = update_post(&db_conn, post_created.id);
            assert_eq!(post_updated.published, true);

            let all_posts = get_posts(&db_conn);
            assert_eq!(all_posts[0].title, post_to_create.title);
            assert_eq!(all_posts[0].body, post_to_create.body);

            delete_post(&db_conn, post_to_create.title);
            let all_posts = get_posts(&db_conn);
            assert_eq!(all_posts.len(), 0);

            Ok(())
        })
    }
}
