table! {
    user (id) {
        id -> Bigint,
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

table! {
    tasks {
        id -> Nullable<Integer>,
        description -> Text,
        completed -> Bool,

    }
}