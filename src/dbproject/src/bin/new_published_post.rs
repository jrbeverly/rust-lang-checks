extern crate dbproject;
extern crate diesel;

use self::diesel::prelude::*;
use self::dbproject::*;
use self::models::Post;
use std::env::args;

fn main() {
    use dbproject::schema::posts::dsl::{posts, published};

    let title = args().nth(1).expect("new_post requires a post title")
        .parse::<String>().expect("Invalid title");
    let body = args().nth(2).expect("new_post requires a post body")
        .parse::<String>().expect("Invalid body");
    let connection = establish_connection();
    
    let post = create_post(&connection, &title, &body);

    let post = diesel::update(posts.find(post.id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", post.id));

    println!("Published post {}", post.id);
}