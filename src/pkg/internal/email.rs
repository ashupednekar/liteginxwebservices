use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::{conf::settings, prelude::Result};
use std::fmt::{self, Display};

pub trait SendEmail {
    fn send(&self, email: &str) -> Result<()>;
}

pub fn send_email(name: &str, email: &str, subject: &str, body: &str, is_html: bool) -> Result<()> {
    let name = name.to_string();
    let email = email.to_string();
    let subject = subject.to_string();
    let body = body.to_string();
    let is_html = is_html;

    tokio::spawn(async move {
        let result = tokio::task::spawn_blocking(move || {
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
                .body(body)
                .unwrap();

            let creds = Credentials::new(settings.smtp_user.clone(), settings.smtp_pass.clone());

            let mailer = SmtpTransport::relay(&settings.smtp_server)
                .unwrap()
                .credentials(creds)
                .build();

            mailer.send(&email)
        })
        .await;

        match result {
            Ok(Ok(_)) => println!("Email sent successfully!"),
            Ok(Err(e)) => eprintln!("Could not send email: {e:?}"),
            Err(e) => eprintln!("Task failed to execute: {e:?}"),
        }
    });
    Ok(())
}

pub struct AuthnCodeTemplate<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

impl<'a> Display for AuthnCodeTemplate<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let html_template = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Verify Your Email</title>
                <style>
                    body {{
                        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
                        line-height: 1.6;
                        margin: 0;
                        padding: 0;
                        background-color: #f9fafb;
                    }}
                    .container {{
                        max-width: 600px;
                        margin: 0 auto;
                        padding: 20px;
                    }}
                    .code-container {{
                        text-align: center;
                        margin: 40px 0;
                        padding: 30px;
                        background-color: #ffffff;
                        border-radius: 8px;
                        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                    }}
                    .verification-code {{
                        font-size: 32px;
                        font-weight: bold;
                        letter-spacing: 4px;
                        color: #059669;
                        margin: 20px 0;
                    }}
                    .message {{
                        color: #4b5563;
                        font-size: 14px;
                        margin: 20px 0;
                    }}
                    .warning {{
                        color: #dc2626;
                        font-size: 12px;
                        margin-top: 20px;
                    }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="code-container">
                        <h2 style="color: #111827; margin: 0;">Your Verification Code</h2>
                        <div class="verification-code">{}</div>
                        <p class="message">
                            This code is for one-time use and will expire in 10 minutes.<br>
                            You'll receive a new code if this one expires.
                        </p>
                        <p class="warning">
                            ⚠️ Do not share this code with anyone.<br>
                            Our team will never ask for this code.
                        </p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            self.code
        );
        write!(f, "{}", html_template)
    }
}

impl<'a> SendEmail for AuthnCodeTemplate<'a> {
    fn send(&self, email: &str) -> crate::prelude::Result<()> {
        send_email(
            &self.name,
            &email,
            "Here's your LWS authentication code",
            &format!("{}", &self),
            true,
        )?;
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use tracing_test::traced_test;

    use super::*;
    use crate::prelude::Result;

    #[test]
    #[traced_test]
    fn test_send_mail_template() -> Result<()> {
        AuthnCodeTemplate {
            name: "Ashu",
            code: "394u93",
        }
        .send("ashupednekar49@gmail.com")?;
        Ok(())
    }
}
