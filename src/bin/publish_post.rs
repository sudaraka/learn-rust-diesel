extern crate diesel;
extern crate learn_rust_diesel;

use std::env::args;

use self::learn_rust_diesel::*;
use self::diesel::prelude::*;

fn main() {
    use learn_rust_diesel::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let conn = db_connection();

    diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&conn)
        .expect(&format!("Unable to find post {}", id));

    let post: models::Post = posts
        .find(id)
        .first(&conn)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}
