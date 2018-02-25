extern crate diesel;
extern crate learn_rust_diesel;

use std::io::{stdin, Read};

use self::learn_rust_diesel::*;

#[cfg(not(windows))]
const EOF: &'static str = "Ctrl+D";

#[cfg(windows)]
const EOF: &'static str = "Ctrl+Z";

fn main() {
    let conn = db_connection();
    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be? ");
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop newline character

    println!(
        "\nOk! Let's write \"{}\" (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    create_post(&conn, title, &body);

    println!("\nSaved draft {}", title);
}
