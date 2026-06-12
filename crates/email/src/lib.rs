//! Pluggable email sender.
//!
//! The `EmailSender` trait abstracts the transport so the auth-ms can be
//! configured per environment (SMTP in production, file-based in dev/tests,
//! and future: SES, SendGrid). Implementations live in [`smtp`] and [`file`].
//!
//! Phase 0a: trait + stubs. Real `lettre`-based SMTP and a `File` impl that
//! writes to `./tmp/emails/<uuid>.eml` arrive in Phase 0b/0c together with
//! the rest of the email surface (templates, password reset, verification).

pub mod file;
pub mod sender;
pub mod smtp;

pub use sender::{EmailError, EmailMessage, EmailSender};
