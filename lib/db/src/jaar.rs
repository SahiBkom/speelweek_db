

struct Jaar {
    pub id: i64,
    pub jaar: u32,
    pub thema: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

struct JaarVrijwilliger {
    jaar_id: i64,
    vrijwilliger_id: i64,
}



