#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate diesel_demo;
use self::diesel_demo::*;

#[get("/")]
fn show_all_posts() -> String {
    match get_posts() {
        Ok(posts) => {
            let mut result = format!("Displaying {} posts\n", posts.len()).to_owned();
            for post in posts {
                result.push_str(
                    &format!("\n{}\n----------\n{}\n", post.title, post.body)
                );
            }
            result
        },
        _ => String::from("Error retrieving posts from DB")
    }
}

#[delete("/delete/<id>")]
fn delete(id: i32) -> String {
    match delete_post_by_id(id) {
        Ok(num) => String::from(format!("Deleted {} post(s)", num)),
        _ => String::from("Error deleting post(s)")
    }
}

fn main() {
    rocket::ignite()
            .mount("/posts", routes![show_all_posts, delete])
            .launch();
}
