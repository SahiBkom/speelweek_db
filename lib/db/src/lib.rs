#![recursion_limit="128"]
#![feature(plugin, decl_macro, custom_derive, const_fn)]


#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate chrono;
#[macro_use] extern crate serde;
extern crate djangohashers;
#[macro_use] extern crate serde_derive;

pub mod schema;
pub mod models;
pub mod task;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use chrono::{Utc, NaiveDateTime};
use djangohashers::{check_password, make_password, Algorithm};
use models::{NewUser, User};
use diesel::prelude::*;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    MysqlConnection::establish(&database_url())
        .expect(&format!("Error connecting to {}", database_url()))
}

pub fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn create_user(conn: &MysqlConnection, username: &str, password: &str) -> User {
    use schema::user::dsl::{id, user};


    let new_user = NewUser {
        username: Some(username.to_string()),
        password: Some(make_password(password)),
    };

    diesel::insert_into(user)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post");

    user.order(id.desc()).first(conn).unwrap()
}


pub fn check_user(conn: &MysqlConnection, username_email_tel: &str, password_in: &str) -> Option<User> {
    use schema::user::dsl::*;

    let results = user
        .filter(username.eq(username_email_tel))
        .or_filter(e_mail_adres.eq(username_email_tel))
        .or_filter(mobiele_nummer.eq(username_email_tel))
        .limit(5)
        .load::<User>(conn)
        .expect("Error loading posts");


    for r in results {
        match check_password(password_in, &r.password.clone().unwrap_or("".to_string())) {
            Ok(valid) => {
                if valid {
                    return Some(r);
                }
            }
            Err(error) => {
//                error!("Error on check password, {}", error);
            }
        }
    }
    // Ask the user to try again.
    None
}