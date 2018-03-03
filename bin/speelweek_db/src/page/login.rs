use std::collections::HashMap;

use rocket::outcome::IntoOutcome;
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Template;
use db;

#[derive(FromForm)]
struct Login {
    username: String,
    password: String
}

#[post("/", data = "<login>")]
fn login(mut cookies: Cookies, login: Form<Login>, conn: db::Conn) -> Flash<Redirect> {
    match db::User::login(&conn, &login.get().username, &login.get().password) {
        Some(user) => {
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            Flash::success(Redirect::to("/"), "Successfully logged in.")
        },
        None => {
            Flash::error(Redirect::to("/login"), "Invalid username/password.")
        }
    }
}

#[get("/logout")]
// #[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/login"), "Successfully logged out.")
}

#[get("/")]
fn login_user(_user: db::user::UserId, conn: db::Conn) -> Redirect {
    println!("login user");
    Redirect::to("/")
}

#[get("/", rank = 2)]
fn login_page(flash: Option<FlashMessage>) -> Template {
    println!("login user 2");
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login", &context)
}

#[get("/home")]
fn user_index(user: db::user::UserId) -> Template {
    println!("login user 3");
    let mut context = HashMap::new();
    context.insert("user_id", user);
    Template::render("index", &context)
}

#[get("/home", rank = 2)]
fn index() -> Redirect {
    Redirect::to("/login")
}