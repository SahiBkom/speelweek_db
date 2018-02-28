use diesel;
use diesel::prelude::*;
use diesel::MysqlConnection;
use chrono::{NaiveDate, NaiveDateTime};
use schema::user;

use schema::user::dsl::{user as all_user};



#[table_name="user"]
#[derive(Queryable, Insertable, Debug, Clone)] // Serialize
pub struct User {
    pub id: i64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub login_at: Option<NaiveDateTime>,
    pub voorletters: Option<String>,
    pub voornaam: Option<String>,
    pub tussenvoegsel: Option<String>,
    pub achternaam: Option<String>,
    pub straatnaam: Option<String>,
    pub huisnummer: Option<String>,
    pub toevoeging_huisnummer: Option<String>,
    pub postcode: Option<String>,
    pub woonplaats: Option<String>,
    pub telefoon_nummer: Option<String>,
    pub mobiele_nummer: Option<String>,
    pub e_mail_adres: Option<String>,
    pub e_mail_toestemming: Option<bool>,
    pub role: Option<i64>,
    pub geboortedatum: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub username: Option<String>,
    pub password: Option<String>,
}


impl User {
    pub fn all(conn: &MysqlConnection) -> Vec<User> {
        all_user.order(user::id.desc()).load::<User>(conn).unwrap()
    }

//    pub fn insert(todo: Todo, conn: &MysqlConnection) -> bool {
//        let t = Task { id: None, description: todo.description, completed: false };
//        diesel::insert_into(tasks::table).values(&t).execute(conn).is_ok()
//    }
//
//    pub fn toggle_with_id(id: i32, conn: &MysqlConnection) -> bool {
//        let task = all_tasks.find(id).get_result::<Task>(conn);
//        if task.is_err() {
//            return false;
//        }
//
//        let new_status = !task.unwrap().completed;
//        let updated_task = diesel::update(all_tasks.find(id));
//        updated_task.set(task_completed.eq(new_status)).execute(conn).is_ok()
//    }

    pub fn delete_with_id(id: i64, conn: &MysqlConnection) -> bool {
        diesel::delete(all_user.find(id)).execute(conn).is_ok()
    }
}
