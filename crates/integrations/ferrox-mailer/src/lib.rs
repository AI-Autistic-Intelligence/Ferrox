//! # Ferrox Mailer (`ferrox-mailer`)
//!
//! `ferrox-mailer` provides async transactional email delivery using `lettre` with support for SMTP, SendGrid, and AWS SES.

use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::str::FromStr;
use ferrox_errors::AppError;

#[derive(Clone)]
pub struct MailerClient {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_address: Mailbox,
}

impl MailerClient {
    /// Initialize the SMTP Mailer (e.g. for SendGrid, AWS SES, Mailgun)
    pub fn new(smtp_server: &str, username: &str, password: &str, from_email: &str, from_name: &str) -> Result<Self, AppError> {
        let creds = Credentials::new(username.to_string(), password.to_string());

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_server)
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?
            .credentials(creds)
            .build();

        let from_address = Mailbox::new(
            Some(from_name.to_string()),
            from_email.parse().map_err(|_| AppError::BadRequest("Invalid sender email".into()))?
        );

        Ok(Self {
            transport,
            from_address,
        })
    }

    /// Sends an HTML email
    pub async fn send_html_email(&self, to_email: &str, to_name: &str, subject: &str, html_body: &str) -> Result<(), AppError> {
        let to_address = Mailbox::new(
            Some(to_name.to_string()),
            to_email.parse().map_err(|_| AppError::BadRequest("Invalid recipient email".into()))?
        );

        let email = Message::builder()
            .from(self.from_address.clone())
            .to(to_address)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_body.to_string())
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        self.transport.send(email)
            .await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        Ok(())
    }
}