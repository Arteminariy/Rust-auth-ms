//! Production [`EmailSender`] backed by `lettre`.
//!
//! Phase 0b: the constructor takes the `from` address directly. In Phase 2
//! `SmtpSender::from_env()` will read `EMAIL__SMTP__HOST` etc. and build a
//! real `AsyncSmtpTransport`. The current `send` only logs the message so
//! `cargo build` and `cargo test` succeed without an SMTP server.

use super::{EmailError, EmailMessage, EmailSender};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use tracing::info;

/// Stub SMTP transport for Phase 0b.
///
/// We avoid the real `lettre::AsyncSmtpTransport` here so the workspace
/// compiles and runs without any network configuration. The trait contract
/// is exercised; the actual wire-format implementation arrives in Phase 2.
#[derive(Clone)]
pub struct SmtpSender {
    #[allow(dead_code)] // surfaced in Phase 2's let::SmtpSender::send
    pub(crate) from: Arc<String>,
    sent: Arc<Mutex<Vec<EmailMessage>>>,
}

impl SmtpSender {
    pub fn new(from: impl Into<String>) -> Self {
        Self {
            from: Arc::new(from.into()),
            sent: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Test-only accessor: snapshot of everything that has been "sent".
    pub fn sent_messages(&self) -> Vec<EmailMessage> {
        self.sent.lock().unwrap().clone()
    }
}

#[async_trait]
impl EmailSender for SmtpSender {
    async fn send(&self, msg: EmailMessage) -> Result<(), EmailError> {
        info!(
            to = %msg.to,
            from = %msg.from,
            subject = %msg.subject,
            "SmtpSender (Phase-0b stub): would deliver"
        );
        self.sent.lock().unwrap().push(msg);
        Ok(())
    }
}
