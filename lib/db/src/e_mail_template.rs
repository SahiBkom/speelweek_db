

struct EmailTemplate {
    pub id: i64,
    pub tag: String,
    pub subject: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
