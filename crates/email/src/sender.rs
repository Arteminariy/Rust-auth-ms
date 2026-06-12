//! The core abstraction for sending transactional email.

use async_trait::async_trait;
use std::collections::HashMap;

/// An outbound email ready to hand to an [`EmailSender`].
///
/// Headers can be used for things like `X-Entity-Ref-ID` for SES, custom
/// `List-Unsubscribe` headers, or arbitrary tags for routing rules.
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub text: String,
    pub html: Option<String>,
    pub headers: HashMap<String, String>,
}

impl EmailMessage {
    pub fn new(
        to: impl Into<String>,
        from: impl Into<String>,
        subject: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            to: to.into(),
            from: from.into(),
            subject: subject.into(),
            text: text.into(),
            html: None,
            headers: HashMap::new(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("transport: {0}")]
    Transport(String),

    #[error("backend not configured: {0}")]
    NotConfigured(String),

    #[error("invalid recipient: {0}")]
    InvalidRecipient(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

/// Send an [`EmailMessage`] through whatever transport the implementor owns.
///
/// Implementors must be `Send + Sync` so a single instance can live inside
/// the axum `AppState` and be shared across all request handlers.
#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, msg: EmailMessage) -> Result<(), EmailError>;
}
