use diesel;
use diesel::prelude::*;
use diesel::MysqlConnection;

use chrono::{NaiveDate, NaiveDateTime, Utc};

use schema::property;
use schema::property::dsl::{property as dsl_property};

#[table_name="property"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, AsChangeset)] //
pub struct Property {
    pub id: i32,
    pub tag: String,
    pub value: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl Property {
    pub fn all(conn: &MysqlConnection) -> Vec<Property> {
        dsl_property.order(property::tag.desc()).load::<Property>(conn).unwrap()
    }

    // TODO: cache this function & and default value
    pub fn by_tag(conn: &MysqlConnection, tag: &str) -> Property {
        dsl_property
            .filter(property::tag.eq(tag))
            .first(conn).unwrap()
    }
}