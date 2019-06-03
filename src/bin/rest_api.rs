#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate diesel_demo;
use self::diesel_demo::*;

#[get("/")]
fn show_all_posts() -> String {
    show_posts()
}

#[delete("/delete/<id>")]
fn delete(id: i32) {
    delete_post_by_id(id);
}

fn main() {
    rocket::ignite()
            .mount("/posts", routes![show_all_posts, delete])
            .launch();
}
