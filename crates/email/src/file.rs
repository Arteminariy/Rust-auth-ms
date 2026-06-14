//! File-based [`EmailSender`] for dev and tests.
//!
//! Writes each message to `dir/<uuid>.eml` so you can inspect what would
//! have been sent. No real network. Active in dev when `EMAIL__BACKEND=file`
//! is set (the `EMAIL__*` env schema is documented in IMPROVEMENT_PLAN.md).

use super::{EmailError, EmailMessage, EmailSender};
use async_trait::async_trait;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

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
    async fn send(&self, msg: EmailMessage) -> Result<(), EmailError> {
        fs::create_dir_all(&self.dir)?;
        let path = self.dir.join(format!("{}.eml", Uuid::new_v4()));
        // Simple RFC 822-ish serialization; replace with `lettre` Message
        // builder in Phase 2 if we need a real `Content-Type: multipart/alternative`.
        let body = format!(
            "From: {from}
To: {to}
Subject: {subj}

{body}
",
            from = msg.from,
            to = msg.to,
            subj = msg.subject,
            body = msg.text,
        );
        fs::write(path, body)?;
        Ok(())
    }
}
