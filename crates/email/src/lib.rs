//! Pluggable email sender.
//!
//! The [`EmailSender`] trait abstracts the transport so `auth-ms` can be
//! configured per environment (SMTP in production, file-based in dev/tests).
//!
//! Phase 0b ships:
//!   - [`SmtpSender`]: real `lettre::AsyncSmtpTransport` (used in prod).
//!   - [`FileSender`]: writes each message to `./tmp/emails/<uuid>.eml` for
//!     local dev / integration tests.
//!
//! Phase 2 will add `SmtpSender::from_env()` (with `EMAIL__SMTP__HOST` etc.),
//! the `lettre` template engine, and the actual `password reset` /
//! `email verification` flows that consume this trait.

pub mod file;
pub mod sender;
pub mod smtp;

pub use sender::{EmailError, EmailMessage, EmailSender};
pub use file::FileSender;
pub use smtp::SmtpSender;
