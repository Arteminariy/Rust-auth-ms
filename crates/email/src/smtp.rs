//! SMTP [`EmailSender`] backed by `lettre`.
//!
//! Phase 0a: stub. The real `lettre::SmtpTransport` wiring + `STARTTLS` /
//! implicit TLS configuration lands in Phase 0b once the rest of the
//! workspace compiles and we can integration-test against `mailhog`.

use super::{EmailError, EmailMessage, EmailSender};
use async_trait::async_trait;

/// SMTP transport. Constructed from env (see Phase 2 schema for keys).
#[derive(Debug, Clone, Default)]
pub struct SmtpSender;

impl SmtpSender {
    pub fn from_env() -> Result<Self, EmailError> {
        // TODO Phase 0b: parse SMTP__HOST / SMTP__PORT / SMTP__USER / SMTP__PASS
        Ok(Self)
    }
}

#[async_trait]
impl EmailSender for SmtpSender {
    async fn send(&self, _msg: EmailMessage) -> Result<(), EmailError> {
        Err(EmailError::NotConfigured(
            "SmtpSender::send is a Phase 0a stub. Real impl lands in Phase 0b.".into(),
        ))
    }
}
