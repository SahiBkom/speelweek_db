use diesel;
use diesel::prelude::*;
use diesel::MysqlConnection;

use chrono::{NaiveDate, NaiveDateTime, Utc};

use schema::emailtemplate;
use schema::emailtemplate::dsl::{emailtemplate as dsl_emailtemplate};

#[table_name="emailtemplate"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, AsChangeset)] //
pub struct EmailTemplate {
    pub id: i32,
    pub tag: String,
    pub subject: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl EmailTemplate {
    pub fn all(conn: &MysqlConnection) -> Vec<EmailTemplate> {
        dsl_emailtemplate.order(emailtemplate::tag.desc()).load::<EmailTemplate>(conn).unwrap()
    }

    // TODO: cache this function & and default value
    pub fn by_tag(conn: &MysqlConnection, tag: &str) -> EmailTemplate {
        dsl_emailtemplate
            .filter(emailtemplate::tag.eq(tag))
            .first(conn).unwrap()
    }
}