extern crate diesel;
extern crate learn_rust_diesel;

use self::learn_rust_diesel::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use learn_rust_diesel::schema::posts::dsl::*;

    let conn = db_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("---------\n");
        println!("{}", post.body);
    }
}
