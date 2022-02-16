extern crate rust_redis;

use redis::Commands;
use rust_redis::*;

fn main() {
    let mut conn = establish_connection();
    let rv: String = conn.get("ankit").unwrap();
    println!("{}",rv);
}
