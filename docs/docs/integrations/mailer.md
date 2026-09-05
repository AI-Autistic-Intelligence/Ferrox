---
sidebar_position: 1
---

# 📧 Email Dispatching (Mailer)

Sending transactional emails (password reset links, account verifications, invoice receipts) is a core requirement for web applications.

`integrations/ferrox-mailer` wraps [Lettre](https://lettre.rs/) to provide an asynchronous SMTP mailer client compatible with SendGrid, AWS SES, Mailgun, and Postmark.

---

## 1. Initializing `MailerClient`

```rust
use ferrox_mailer::MailerClient;
use ferrox_errors::AppError;

let mailer = MailerClient::new(
    "smtp.sendgrid.net",
    "apikey",
    "YOUR_SENDGRID_API_KEY",
    "noreply@mycompany.com",
    "Ferrox Platform"
)?;
```

---

## 2. Sending HTML Emails

```rust
pub async fn send_welcome_email(mailer: &MailerClient, to_email: &str, user_name: &str) -> Result<(), AppError> {
    let subject = "Welcome to Ferrox!";
    let html_body = format!(
        "<h1>Welcome, {}!</h1><p>Thank you for joining our platform.</p>",
        user_name
    );

    mailer.send_html_email(to_email, user_name, subject, &html_body).await?;
    println!("✅ Welcome email dispatched to {}", to_email);
    Ok(())
}
```

---

## 3. ✅ Best Practices

- **Offload Email Dispatch to Background Workers**: Always invoke `MailerClient` inside a `ferrox-jobs` background worker so email latency never blocks HTTP response times.
