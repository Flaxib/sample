extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
// use self::models::*;
// use std::env::args;

fn main() {
    use diesel_demo::schema::posts::dsl::*;

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts)
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
