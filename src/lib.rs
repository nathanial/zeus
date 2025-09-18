//! Core library for the Zeus language runtime and tooling.

pub mod cli;
pub mod introspection;
pub mod language;
pub mod runtime;

pub use crate::runtime::ZeusRuntime;

/// Common result type used across the Zeus runtime layers.
pub type ZeusResult<T> = Result<T, ZeusError>;

/// Minimal error type until the runtime grows richer error reporting.
#[derive(Debug)]
pub enum ZeusError {
    /// Generic bootstrap or configuration failure.
    Bootstrap(String),
}

impl std::fmt::Display for ZeusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZeusError::Bootstrap(message) => write!(f, "bootstrap failure: {message}"),
        }
    }
}

impl std::error::Error for ZeusError {}
