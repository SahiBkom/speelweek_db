//--
//-- Table structure for table `team`
//--
//
//CREATE TABLE IF NOT EXISTS `team` (
//`id` bigint(20) NOT NULL,
//`naam` varchar(255) DEFAULT NULL,
//`omschrijving` varchar(255) DEFAULT NULL,
//`voorbereiding` tinyint(1) DEFAULT '0',
//`middagprogramma` tinyint(1) DEFAULT '0',
//`when_created` datetime NOT NULL,
//`when_updated` datetime NOT NULL
//) ENGINE=InnoDB DEFAULT CHARSET=utf8;
//
//
//
//CREATE TABLE IF NOT EXISTS `team_vrijwilliger` (
//`id` bigint(20) NOT NULL,
//`team_id` bigint(20) NOT NULL,
//`vrijwilliger_id` bigint(20) NOT NULL,
//`programma` tinyint(1) DEFAULT '0',
//`voorbereiding` tinyint(1) DEFAULT '0',
//`when_created` datetime NOT NULL,
//`when_updated` datetime NOT NULL
//) ENGINE=InnoDB  DEFAULT CHARSET=utf8;
//
//

use diesel;
use diesel::prelude::*;
use diesel::MysqlConnection;

use chrono::{NaiveDate, NaiveDateTime, Utc};

//use rocket::http::{Cookie, Cookies};
//use rocket::outcome::IntoOutcome;
////use rocket::request::{Form, FromFormValue};
//use rocket::request::{self, Form, FromFormValue, FromRequest, Request};
//use rocket::response::Redirect;

use schema::team;
use schema::team::dsl::{team as dsl_team};

use schema::team_vrijwilliger;
use schema::team_vrijwilliger::dsl::{team_vrijwilliger as dsl_team_vrijwilliger};


#[table_name="team"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, AsChangeset)] //
pub struct Team {
    pub id: i32, // TODO change to u32 after https://github.com/diesel-rs/diesel/pull/1561
    pub naam: String,
    pub omschrijving: String,
    pub voorbereiding: bool,
    pub middagprogramma: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl Team {
    pub fn all(conn: &MysqlConnection) -> Vec<Team> {
        dsl_team.order(team::naam.desc()).load::<Team>(conn).unwrap()
    }

    pub fn all_voorbereiding(conn: &MysqlConnection) -> Vec<Team> {
        dsl_team
            .filter(team::voorbereiding.eq(true))
            .order(team::naam.desc())
            .load::<Team>(conn).unwrap()
    }

    pub fn all_middagprogramma(conn: &MysqlConnection) -> Vec<Team> {
        dsl_team
            .filter(team::middagprogramma.eq(true))
            .order(team::naam.desc())
            .load::<Team>(conn).unwrap()
    }

//    pub fn insert(todo: String, conn: &MysqlConnection) -> bool {
//        let t = Task { id: None, description: todo, completed: false };
//        diesel::insert_into(team::table).values(&t).execute(conn).is_ok()
//    }


    pub fn delete_with_id(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(dsl_team.find(id)).execute(conn).is_ok()
    }

}

#[table_name="team_vrijwilliger"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, AsChangeset)] //
struct TeamVrijwilliger {
    pub id: i32,
    pub team_id: i32,
    pub vrijwilliger_id: i32,
    pub programma: bool,
    pub voorbereiding: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}