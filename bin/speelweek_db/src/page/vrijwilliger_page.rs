use rocket::response::Redirect;
use rocket::request::{Form, FromFormValue};
use rocket::http::RawStr;
use rocket_contrib::Template;
use login;
use db::{self, user, team};


#[get("/", rank = 1)]
fn index(conn: db::Conn, userId: user::UserId) -> Template {

    #[derive(Debug, Serialize)]
    struct Page {
        user: user::User,
        middagprogramma: Vec<team::Team>,
    }

    let user = user::User::get_by_id(&conn, userId);
    let middagprogramma = team::Team::all_middagprogramma(&conn);

    let data = Page { user, middagprogramma};

    Template::render("vrijwilliger", data)
}
