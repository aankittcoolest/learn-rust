extern crate rust_redis;

use redis::Commands;
use rust_redis::*;

fn main() {
    let mut conn = establish_connection();
    let _: () = conn.set("ankit", "agrawal").expect(&format!("unable to set the key"));
    println!("Created key successfully!")
}