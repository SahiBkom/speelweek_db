#![recursion_limit="128"]
#![feature(plugin, decl_macro, custom_derive, const_fn)]


#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate chrono;
#[macro_use] extern crate serde;
extern crate djangohashers;
#[macro_use] extern crate serde_derive;

extern crate rocket;

pub mod schema;
pub mod user;
pub mod task;
pub mod team;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use chrono::{Utc, NaiveDateTime};
use djangohashers::{check_password, make_password, Algorithm};
use user::NewUser;
use diesel::prelude::*;

use std::ops::Deref;

use diesel::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};


use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};




pub use user::User;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    MysqlConnection::establish(&database_url())
        .expect(&format!("Error connecting to {}", database_url()))
}

pub fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub type SqlitePool = Pool<ConnectionManager<MysqlConnection>>;


//pub fn database_url() -> String {
//    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
//}

pub fn init_pool() -> SqlitePool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

pub struct Conn(pub PooledConnection<ConnectionManager<MysqlConnection>>);

impl Deref for Conn {
    type Target = MysqlConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<SqlitePool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
