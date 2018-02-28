extern crate diesel;
extern crate db;
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use db::*;
use diesel::MysqlConnection;
use db::models::User;
use diesel::prelude::*;

fn main() {
    let connection = establish_connection();

//    let cuser = create_user(&connection, &"Sahi7", &"password");
//    println!("\nSaved draft {:?} with id {}", cuser.username, cuser.id);

    match check_user(&connection, &"Sahi8", &"password") {
        Some(u) => println!("login {:?}", u.username),
        None => println!("Error on login"),
    }

    show(connection);

}


fn show(connection: MysqlConnection) {
    use db::schema::user::dsl::*;
    let results = user
//        .filter(published.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{} {} {}", post.id, post.username.unwrap_or("".to_string()), post.password.unwrap_or("".to_string()));
    }
}