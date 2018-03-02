use rocket::response::Redirect;
use rocket::request::{Form, FromFormValue};
use rocket::http::RawStr;

use login;
use db;
use f_db;

use chrono::NaiveDate;


#[derive(Debug)]
enum Geslacht {
    M,
    V,
    O
}

impl<'v> FromFormValue<'v> for Geslacht {
    type Error = &'v RawStr;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        let variant = match v.as_str() {
            "M" | "m" => Geslacht::M,
            "V" | "v" => Geslacht::V,
            "O" => Geslacht::O,
            _ => return Err(v)
        };

        Ok(variant)
    }
}

#[derive(Debug)]
enum Kledingmaat {
    XS,
    S,
    M,
    L,
    XL,
    XXL,
    XXXL,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum FromBool {
    True,
    False
}

impl<'v> FromFormValue<'v> for FromBool {
    type Error = &'v RawStr;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        let variant = match v.as_str() {
            "Ja" | "Ja" | "True" | "true" | "On" | "on" => FromBool::True,
            "Nee" | "nee" | "False" | "false" | "Off" | "off" => FromBool::False,
            _ => return Err(v)
        };

        Ok(variant)
    }
}


impl FromBool {
    pub fn to_bool(self) -> bool {
        self == FromBool::True
    }
}

//#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
//struct Email<'r>(Option<&'r str>);
//
//impl<'v> FromFormValue<'v> for Email<> {
//    type Error = &'static str;
//
//    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
//        match v.len() {
//            0 => return Ok(Email(None)),
//            1...8 => Err("too short!"),
//            _ => {
//                match v.url_decode() {
//                    Ok(email) => Ok(Email(Some(email))),
//                    Err(e) => Err("Error decoding string"),
//                }
//            }
//        }
//    }
//}

#[derive(Debug)]
struct Tel<'r>(&'r str);

impl<'v> FromFormValue<'v> for Tel<'v> {
    type Error = &'static str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        // TODO: check tel
        if (v.len() > 0) && (v.len() < 8) {
            Err("too short or a tel!")
        } else {
            Ok(Tel(v.as_str()))
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct FromDate(Option<NaiveDate>);

impl<'v> FromFormValue<'v> for FromDate {
    type Error = &'static str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        let date = match v.url_decode() {
            Ok(ref e) if e.is_empty() => return Ok(FromDate(None)),
            Ok(e) => e,
            Err(e) => return Err("Error formate date string"),
        };

        match NaiveDate::parse_from_str(&date, "%d-%m-%Y") {
            Ok(date) => Ok(FromDate(Some(date))),
            Err(e) => Err("Error paring date")
        }
    }
}



#[derive(FromForm, Debug)]
struct UserForm<'r> {
    pub achternaam: &'r RawStr,
    pub dieten: &'r RawStr,
    pub e_mail_adres: &'r RawStr,
    pub e_mail_toestemming: FromBool,
    pub geboortedatum: FromDate,
    pub geslacht: Geslacht,
    pub huisnummer: &'r RawStr,
    pub kleding_maat: &'r RawStr,
    pub mobiele_nummer: &'r RawStr,
    pub password_a: &'r RawStr,
    pub password_b: &'r RawStr,
    pub postcode: &'r RawStr,
    pub straatnaam: &'r RawStr,
    pub telefoon_nummer: &'r RawStr,
    pub toevoeging_huisnummer: &'r RawStr,
    pub tussenvoegsel: &'r RawStr,
    pub username: &'r RawStr,
    pub voorletters: &'r RawStr,
    pub voornaam: &'r RawStr,
    pub woonplaats: &'r RawStr,
}

#[post("/save", data = "<user_form>")]
fn save<'a>(
    conn: db::Conn,
    user_id:
    login::UserId,
    user_form:Form<'a, UserForm<'a>>
) -> String {

    let user_form = user_form.get();

    let mut user = f_db::User::get_by_id(&conn, user_id.0 as i64);

    user.achternaam = Some(user_form.achternaam.url_decode().unwrap());
//    user.e_mail_adres = user_form.e_mail_adres.0;
    user.e_mail_toestemming = Some(user_form.e_mail_toestemming.to_bool()); //TODO: Change to boolean remove some
    user.geboortedatum = user_form.geboortedatum.0;
    user.huisnummer = Some(user_form.huisnummer.url_decode().unwrap());
    user.mobiele_nummer = Some(user_form.mobiele_nummer.url_decode().unwrap());
//    pub password: Option<String>,
    user.postcode = Some(user_form.postcode.url_decode().unwrap());
    user.straatnaam = Some(user_form.straatnaam.url_decode().unwrap());
    user.telefoon_nummer = Some(user_form.telefoon_nummer.url_decode().unwrap());
    user.toevoeging_huisnummer = Some(user_form.toevoeging_huisnummer.url_decode().unwrap());
    user.tussenvoegsel = Some(user_form.tussenvoegsel.url_decode().unwrap());
    user.username = Some(user_form.username.url_decode().unwrap());
    user.voorletters = Some(user_form.voorletters.url_decode().unwrap());
    user.voornaam = Some(user_form.voornaam.url_decode().unwrap());
    user.woonplaats = Some(user_form.woonplaats.url_decode().unwrap());

    println!("update:{:?}", f_db::User::update(&conn, &user));

    println!("Form resutl is {:?}\n{:?}", user, user_form);
    format!("Form resutl is {:?}", user)
}