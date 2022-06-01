extern crate diesel;
extern crate diesel_demo;

use self::diesel_demo::*;
use rand::{distributions::Alphanumeric, Rng};
use std::env::args;

fn generate_string(length: i32) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect::<String>()
}

fn main() {
    let nb_post_to_generate = args()
        .nth(1)
        .expect("write_posts requires a number of posts to write")
        .parse::<u32>()
        .expect("Invalid positive integer");

    println!("Number of posts to generate {}", nb_post_to_generate);

    let connection = establish_connection();
    for _ in 0..nb_post_to_generate {
        let title = generate_string(7);
        // println!("{}", title);
        let body = generate_string(30);
        // println!("{}", body);
        let p = create_post(&connection, &title, &body);
        println!("Saved draft {} with id {}", title, p.id);
    }
}
