use std::env;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::Error;
use lettre::transport::smtp::response::Response;

pub fn send_email(email: Message) -> Result<Response, Error> {
    let smtp_user = env::var("MAIL_USER").expect("MAIL_USER must be set");
    let smtp_password = env::var("MAIL_PASSWORD").expect( "MAIL_PASSWORD must be set");
    let smtp_host = env::var("MAIL_HOST").expect("MAIL_HOST must be set");
    let creds = Credentials::new(smtp_user, smtp_password);

    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email)
}