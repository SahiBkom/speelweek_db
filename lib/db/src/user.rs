use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::MysqlConnection;
use diesel::prelude::*;
use diesel;
use djangohashers::{make_password, check_password};
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
//use rocket::request::{Form, FromFormValue};
use rocket::request::{self, Form, FromFormValue, FromRequest, Request};
use rocket::response::Redirect;
use schema::user::dsl::{user as dsl_user};
use schema::user;

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct UserId(i32);

impl<'a, 'r> FromRequest<'a, 'r> for UserId {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserId, ()> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| UserId(id))
            .or_forward(())
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Role {
    Vrijwileger,
    Ouder,
    AdminRead,
    AdminWrite
}


impl Role {

    fn id(self) -> u32 {
        match self {
            Role::Vrijwileger => 1,
            Role::Ouder => 2,
            Role::AdminRead => 3,
            Role::AdminWrite => 4,
        }
    }

    fn mask(self) -> u64 {
        1u64 >> self.id()
    }

    pub fn is_check(self, set: u64) -> bool {
        set & self.mask() == self.mask()
    }

}


#[table_name="user"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, AsChangeset)] //
pub struct User {
    pub id: i32,
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
        dsl_user.order(user::id.desc()).load::<User>(conn).unwrap()
    }

    pub fn get_by_id(conn: &MysqlConnection, userId: UserId) -> User {
        dsl_user.find(userId.0).first::<User>(conn).unwrap()
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


    pub fn update_login_time(conn: &MysqlConnection, userId: UserId) -> bool {
        let sql = diesel::update(dsl_user.filter(user::id.eq(userId.0)))
            .set(user::login_at.eq(Some(Utc::now().naive_utc()))).execute(conn);

        match sql {
            Err(e) => {
                println!("Error on User::update_login_time {:?}", e);
                false
            },
            Ok(_) => true,
        }
    }

    pub fn update(conn: &MysqlConnection, user_in: &User) -> Result<usize, String> {
        match diesel::update(user::table).filter(user::id.eq(user_in.id)).set(user_in).execute(conn) {
            Ok(v) => Ok(v),
            Err(e) => {
                println!("error on update {:?}, error: {:?}",user_in, e);
                Err(e.to_string())
            }
        }
    }

    pub fn delete_with_id(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(dsl_user.find(id)).execute(conn).is_ok()
    }


    pub fn create(conn: &MysqlConnection, username: &str, password: &str) -> Self {
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


    pub fn login(conn: &MysqlConnection, username_email_tel: &str, password_in: &str) -> Option<Self> {
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
                    User::update_login_time(conn, UserId(r.id));
                    return Some(r);
                },
                Err(error) => {
//                error!("Error on check password, {}", error);
                }
            }
        }
        // Ask the user to try again.
        None
    }
}
