use lettre::{
    message::{header::ContentType, Mailbox as LettreMailbox, Message},
    transport::smtp::{authentication::Credentials, client::{Tls, TlsParameters}},
    AsyncSmtpTransport, AsyncTransport,
    Tokio1Executor,
};

use crate::entities::settings_model::EmailSettings;

#[allow(dead_code)]
pub async fn send_email(
    settings: &EmailSettings,
    to_email: &str,
    to_name: Option<&str>,
    subject: &str,
    body: &str,
) -> Result<(), String> {
    // 1. Validate settings
    if settings.smtp_server.is_empty() || settings.smtp_username.is_empty() || settings.smtp_password.is_empty() {
        return Err("SMTP settings are incomplete.".to_string());
    }

    // 2. Create email message
    let to_address = to_email.parse().map_err(|e| format!("Invalid recipient email: {}", e))?;
    let from_address = settings.from_email.parse().map_err(|e| format!("Invalid sender email: {}", e))?;

    let to_mailbox = LettreMailbox::new(to_name.map(|s| s.to_string()), to_address);
    let from_mailbox = LettreMailbox::new(Some(settings.from_name.clone()), from_address);

    let email = Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .map_err(|e| format!("Failed to build email: {}", e))?;

    // 3. Setup SMTP client
    let creds = Credentials::new(settings.smtp_username.clone(), settings.smtp_password.clone());

    let tls_parameters = TlsParameters::new(settings.smtp_server.clone())
        .map_err(|e| format!("Failed to build TLS parameters: {}", e))?;
    let tls = Tls::Required(tls_parameters);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(settings.smtp_server.clone())
        .port(settings.smtp_port as u16)
        .tls(tls)
        .credentials(creds)
        .build();

    // 4. Send email
    match mailer.send(email).await {
        Ok(_) => {
            println!("Email sent successfully to {}", to_email);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to send email: {}", e);
            Err(format!("Failed to send email: {}", e))
        }
    }
}