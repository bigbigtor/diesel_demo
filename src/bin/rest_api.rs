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
        _ => String::from("asdf")
    }
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
