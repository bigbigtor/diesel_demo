pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Post, NewPost};

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_posts() -> QueryResult<Vec<Post>> {
    use crate::schema::posts::dsl::*;
    let connection = establish_connection();
    posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
}

pub fn delete_post_by_id (post_id: i32) -> QueryResult<usize> {
    use crate::schema::posts::dsl::*;
    let connection = establish_connection();
    diesel::delete(
        posts.filter(
            id.eq(post_id)
        )
    ).execute(&connection)
}
