extern crate lettre;
extern crate lettre_email;
extern crate db;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{SimpleSendableEmail, EmailTransport, EmailAddress, SmtpTransport};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;
use lettre_email::EmailBuilder;
use std::collections::HashMap;
use db::EmailTemplate;

struct Email {
    property: HashMap<String, String>,
    template: HashMap<String, EmailTemplate>,
    mailer: SmtpTransport,
}

impl Email {

    //TODO create new_db(db)

    fn new(property: HashMap<String, String>, template: HashMap<String, EmailTemplate>) -> Self {

        let smtp_host = property.get("mail.smtp.host").expect("Missing property mail.smtp.host").clone();
        let login_user = property.get("mail.login.user").expect("Missing property mail.login.user").clone();
        let login_pass = property.get("mail.login.pass").expect("Missing property mail.login.pass").clone();

        let mut mailer = SmtpTransport::simple_builder(smtp_host).unwrap()
            .credentials(Credentials::new(login_user, login_pass))
            .smtp_utf8(true)
            .authentication_mechanism(Mechanism::CramMd5)
            .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).build();

        Email{property, template, mailer}
    }

    fn send(&mut self, tag: &str, values: Vec<(String, String)>, to: &str) {
        let tp: &EmailTemplate = self.template.get(tag).expect("Missing template");

        let subject = &tp.subject;
        let text = &tp.text; //TODO replace values
        let from = self.property.get("mail.send.from").expect("Missing property mail.send.from").clone();

        let email = EmailBuilder::new()
            .to(to)
            .from(from)
            .subject(subject.clone())
            .text(text.clone())
            .build().unwrap();

        println!("{:?}",  self.mailer.send(&email));
    }

}

/// Explicitly close the SMTP transaction as we enabled connection reuse
impl Drop for Email {
    fn drop(&mut self) {
        self.mailer.close();
    }
}
