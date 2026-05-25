//! Library root for lol_wiki_md.
//!
//! Exposes high-level conversion functions plus lower-level parsing utilities.
//! Enforces safety guarantees (no `unsafe` blocks permitted in core crate).

#![deny(unsafe_code)]

pub mod cli;
pub mod convert;
pub mod error;
pub mod model;
pub mod parse;
pub mod render;
pub mod similarity;
pub mod validate;
pub mod wiki_export;

pub use crate::cli::CliConfig;
pub use crate::convert::{
    convert_champion, convert_item, convert_rune, ConversionContext, ConversionOutcome,
};
pub use crate::error::{ConvertError, Result};
pub use crate::validate::validate_templates;
