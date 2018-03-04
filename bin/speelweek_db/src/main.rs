#![recursion_limit="128"]
#![feature(plugin, decl_macro, custom_derive, const_fn)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate chrono;
extern crate dotenv;
extern crate db;

mod static_files;
mod page;
#[cfg(test)] mod tests;

use rocket::Rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;
use dotenv::dotenv;

#[derive(FromForm)]
pub struct Todo {
    pub description: String,
}


#[get("/", rank = 2)]
fn index(conn: db::Conn, user: db::user::UserId) -> Template {
    println!("test 3");
    Template::render("index", user)
}


fn rocket() -> (Rocket, Option<db::Conn>) {
    let pool = db::init_pool();
    let conn = if cfg!(test) {
        Some(db::Conn(pool.get().expect("database connection for testing")))
    } else {
        None
    };

    let rocket = rocket::ignite()
        .manage(pool)
        .mount("/", routes![index, static_files::all])
        .mount("/login/", routes![page::login::index, page::login::user_index,
            page::login::login, page::login::logout, page::login::login_user, page::login::login_page])
        .mount("/user/", routes![page::user_page::save, page::user_page::index])
        .mount("/vrijwilliger/", routes![page::vrijwilliger_page::index, ])
        .attach(Template::fairing());

    (rocket, conn)
}

fn main() {
    dotenv().ok();
    println!("DB = {} ", db::database_url());
    rocket().0.launch();
}
