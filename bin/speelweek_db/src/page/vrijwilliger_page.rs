


use rocket_contrib::Template;

use db::{self, user, team};

#[derive(Debug, Serialize)]
struct Opgave {
    pub id: i32,
    pub naam: String,
    pub omschrijving: String,
    pub opgave: bool,
}


#[get("/", rank = 1)]
fn index(conn: db::Conn, userId: user::UserId) -> Template {

    #[derive(Debug, Serialize)]
    struct Page {
        user: user::User,
        middagprogramma: Vec<Opgave>,
        voorbereiding: Vec<Opgave>,
    }

    let user = user::User::get_by_id(&conn, userId);

    //TODO change to sql?
    let opgaven: Vec<i32> = team::TeamVrijwilliger::all_user(&conn, userId)
        .into_iter().filter(|f| f.programma)
        .map(|m| m.id).collect();

    let middagprogramma = team::Team::all_middagprogramma(&conn).into_iter()
        .map(|team| {
            let op =  match opgaven.iter().find(| &&i| i == team.id) {
                Some(_) => true,
                None => false,
            };
            Opgave {
                id: team.id,
                naam: team.naam,
                omschrijving: team.omschrijving,
                opgave: op,
            }
        })
        .collect();

    //TODO change to sql?
    let opgaven:Vec<i32> = team::TeamVrijwilliger::all_user(&conn, userId)
        .into_iter().filter(|f| f.voorbereiding)
        .map(|m| m.id).collect();

    let voorbereiding = team::Team::all_voorbereiding(&conn).into_iter()
        .map(|team| {
            let op =  match opgaven.iter().find(| &&i| i == team.id) {
                Some(_) => true,
                None => false,
            };
            Opgave {
                id: team.id,
                naam: team.naam,
                omschrijving: team.omschrijving,
                opgave: op,
            }
        })
        .collect();

    Template::render("vrijwilliger", Page { user, middagprogramma, voorbereiding})
}
