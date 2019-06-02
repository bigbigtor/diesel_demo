#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate diesel_demo;
use self::diesel_demo::*;

#[get("/posts")]
fn show_all_posts() -> String {
    show_posts()
}

fn main() {
    rocket::ignite()
            .mount("/", routes![show_all_posts])
            .launch();
}
