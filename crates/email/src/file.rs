//! File-based [`EmailSender`] for dev and tests.
//!
//! Writes each message to `./tmp/emails/<uuid>.eml` so you can inspect what
//! would have been sent. No real network. Active in dev when
//! `EMAIL__BACKEND=file` is set (the `EMAIL__*` env schema is documented in
//! Phase 2 of IMPROVEMENT_PLAN.md).

use super::{EmailError, EmailMessage, EmailSender};
use async_trait::async_trait;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileSender {
    pub dir: PathBuf,
}

impl FileSender {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }
}

impl Default for FileSender {
    fn default() -> Self {
        Self::new("./tmp/emails")
    }
}

#[async_trait]
impl EmailSender for FileSender {
    async fn send(&self, _msg: EmailMessage) -> Result<(), EmailError> {
        // TODO Phase 0b: serialize msg to RFC 822 .eml, write to self.dir/<uuid>.eml
        Err(EmailError::NotConfigured(
            "FileSender::send is a Phase 0a stub. Real impl lands in Phase 0b.".into(),
        ))
    }
}
