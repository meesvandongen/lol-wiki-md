//! Library root for lol_wiki_md.
//!
//! Exposes high-level conversion functions plus lower-level parsing utilities.
//! Enforces safety guarantees (no `unsafe` blocks permitted in core crate).

#![deny(unsafe_code)]

pub mod cli;
pub mod error;
pub mod model;
pub mod convert;
pub mod parse;
pub mod render;
pub mod wiki_export;
pub mod validate;

pub use crate::cli::CliConfig;
pub use crate::convert::{convert_champion, ConversionOutcome};
pub use crate::error::{ConvertError, Result};
pub use crate::validate::validate_templates;
