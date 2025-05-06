use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::{conf::settings, prelude::Result};

pub trait SendEmail {
    fn send(&self, email: &str) -> Result<()>;
}

pub fn send_email(name: &str, email: &str, subject: &str, body: &str, is_html: bool) -> Result<()> {
    tracing::debug!("settings: {:?}", *settings);

    let content_type = if is_html {
        ContentType::TEXT_HTML
    } else {
        ContentType::TEXT_PLAIN
    };

    let email = Message::builder()
        .from(
            format!("{} <{}>", &settings.service_name, &settings.from_email)
                .parse()
                .unwrap(),
        )
        .to(format!("{} <{}>", &name, &email).parse().unwrap())
        .subject(subject)
        .header(content_type)
        .body(String::from(body))
        .unwrap();

    let creds = Credentials::new(settings.smtp_user.clone(), settings.smtp_pass.clone());

    let mailer = SmtpTransport::relay(&settings.smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }

    Ok(())
}
