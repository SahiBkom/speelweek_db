table! {
    tasks {
        id -> Nullable<Integer>,
        description -> Text,
        completed -> Bool,

    }
}
table! {
    team (id) {
        id -> Integer,
        naam -> Varchar,
        omschrijving -> Varchar,
        voorbereiding -> Bool,
        middagprogramma -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    team_vrijwilliger (id) {
        id -> Integer,
        team_id -> Integer,
        user_id -> Integer,
        programma -> Bool,
        voorbereiding -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    user (id) {
        id -> Integer,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        login_at -> Nullable<Datetime>,
        voorletters -> Nullable<Varchar>,
        voornaam -> Nullable<Varchar>,
        tussenvoegsel -> Nullable<Varchar>,
        achternaam -> Nullable<Varchar>,
        straatnaam -> Nullable<Varchar>,
        huisnummer -> Nullable<Varchar>,
        toevoeging_huisnummer -> Nullable<Varchar>,
        postcode -> Nullable<Varchar>,
        woonplaats -> Nullable<Varchar>,
        telefoon_nummer -> Nullable<Varchar>,
        mobiele_nummer -> Nullable<Varchar>,
        e_mail_adres -> Nullable<Varchar>,
        e_mail_toestemming -> Nullable<Bool>,
        role -> Nullable<Bigint>,
        geboortedatum -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    tasks,
    team,
    team_vrijwilliger,
    user,
);
