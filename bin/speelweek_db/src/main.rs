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
mod login;
mod page;
#[cfg(test)] mod tests;

use rocket::Rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;
use dotenv::dotenv;
use db::task::Task;
use page::user_page;


#[derive(FromForm)]
pub struct Todo {
    pub description: String,
}


#[derive(Debug, Serialize)]
struct ContextMsg<'a, 'b>{ msg: Option<(&'a str, &'b str)>, tasks: Vec<Task> }

impl<'a, 'b> ContextMsg<'a, 'b> {
    pub fn err(conn: &db::Conn, msg: &'a str) -> ContextMsg<'static, 'a> {
        ContextMsg {msg: Some(("error", msg)), tasks: Task::all(conn)}
    }

    pub fn raw(conn: &db::Conn, msg: Option<(&'a str, &'b str)>) -> ContextMsg<'a, 'b> {
        ContextMsg {msg: msg, tasks: Task::all(conn)}
    }
}

#[post("/", data = "<todo_form>")]
fn new(todo_form: Form<Todo>, conn: db::Conn, user: login::UserId) -> Flash<Redirect> {
    let todo = todo_form.into_inner();
    if todo.description.is_empty() {
        Flash::error(Redirect::to("/"), "Description cannot be empty.")
    } else if Task::insert(todo.description, &conn) {
        Flash::success(Redirect::to("/"), "Todo successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Whoops! The server failed.")
    }
}

#[put("/<id>")]
fn toggle(id: i32, conn: db::Conn, user: login::UserId) -> Result<Redirect, Template> {
    if Task::toggle_with_id(id, &conn) {
        Ok(Redirect::to("/"))
    } else {
        Err(Template::render("index", &ContextMsg::err(&conn, "Couldn't toggle task.")))
    }
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Conn, user: login::UserId) -> Result<Flash<Redirect>, Template> {
    if Task::delete_with_id(id, &conn) {
        Ok(Flash::success(Redirect::to("/"), "Todo was deleted."))
    } else {
        Err(Template::render("index", &ContextMsg::err(&conn, "Couldn't delete task.")))
    }
}


#[get("/", rank = 2)]
fn index(msg: Option<FlashMessage>, conn: db::Conn, user: login::UserId) -> Template {
    println!("test 3");
    Template::render("index", &match msg {
        Some(ref msg) => ContextMsg::raw(&conn, Some((msg.name(), msg.msg()))),
        None => ContextMsg::raw(&conn, None),
    })
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
        .mount("/todo/", routes![new, toggle, delete])
        .mount("/login/", routes![login::index, login::user_index, login::login, login::logout, login::login_user, login::login_page])
        .mount("/user/", routes![page::user_page::save, page::user_page::index])
        .attach(Template::fairing());

    (rocket, conn)
}

fn main() {
    dotenv().ok();
    println!("DB = {} ", db::database_url());
    rocket().0.launch();
}
