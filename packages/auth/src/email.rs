//! Email provider abstraction for verification, magic links, and OTPs.

use async_trait::async_trait;

/// An email message to send.
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
}

/// Trait for sending transactional auth emails.
#[async_trait]
pub trait EmailProvider: Send + Sync + 'static {
    /// Send an email. Implementations may log, SMTP, or queue.
    async fn send(&self, message: EmailMessage) -> anyhow::Result<()>;
}

/// Development provider that logs emails to tracing / stdout.
#[derive(Debug, Default, Clone)]
pub struct ConsoleEmailProvider;

impl ConsoleEmailProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EmailProvider for ConsoleEmailProvider {
    async fn send(&self, message: EmailMessage) -> anyhow::Result<()> {
        tracing::info!(
            to = %message.to,
            subject = %message.subject,
            body = %message.body_text,
            "auth email (console)"
        );
        println!(
            "[montrs-auth email] to={} subject={}\n{}",
            message.to, message.subject, message.body_text
        );
        Ok(())
    }
}

/// No-op email provider for tests.
#[derive(Debug, Default, Clone)]
pub struct NullEmailProvider;

#[async_trait]
impl EmailProvider for NullEmailProvider {
    async fn send(&self, _message: EmailMessage) -> anyhow::Result<()> {
        Ok(())
    }
}
