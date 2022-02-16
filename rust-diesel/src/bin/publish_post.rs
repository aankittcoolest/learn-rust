extern crate rust_diesel;
extern crate diesel;

use self::diesel::prelude::*;
use self::rust_diesel::*;
use std::env::args;

fn main() {
    use rust_diesel::schema::posts::dsl::{posts, published};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let conn = establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&conn);
    println!("Post updated successfully!");
}