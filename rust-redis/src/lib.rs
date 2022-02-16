extern crate redis;

use redis::{Connection};

pub fn establish_connection() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1/").expect(&format!("Error connect to redis on {}", "redis://127.0.0.1/"));
    return client.get_connection().expect(&format!("Failed to establish connection"));
}