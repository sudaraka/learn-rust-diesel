extern crate diesel;
extern crate learn_rust_diesel;

use std::env::args;

use self::learn_rust_diesel::*;
use self::diesel::prelude::*;

fn main() {
    use learn_rust_diesel::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");

    let pattern = format!("%{}%", target);

    let conn = db_connection();

    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&conn)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
