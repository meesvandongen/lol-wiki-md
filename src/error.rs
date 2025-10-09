use thiserror::Error;
use std::io;

pub type Result<T, E = ConvertError> = std::result::Result<T, E>;

/// Stable error codes (stringified). Keep additions append-only.
#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("E_CHAMPION_NOT_FOUND: {0}")]
    ChampionNotFound(String),
    #[error("E_ITEM_NOT_FOUND: {0}")]
    ItemNotFound(String),
    #[error("E_RUNE_NOT_FOUND: {0}")]
    RuneNotFound(String),
    #[error("E_LUA_PARSE: {detail}")]
    LuaParse { detail: String },
    #[error("E_MALFORMED_TEMPLATE {name}: {detail}")]
    MalformedTemplate { name: String, detail: String },
    #[error("E_UNBALANCED_BRACES {name}")]
    UnbalancedBraces { name: String },
    #[error("E_UNKNOWN_TEMPLATE {name}")]
    UnknownTemplate { name: String },
    #[error("E_UNKNOWN_TEMPLATE_SPAN {name} at {line}:{col}")]
    UnknownTemplateSpan { name: String, line: usize, col: usize },
    #[error("E_EXPR {expr}: {detail}")]
    Expr { expr: String, detail: String },
    #[error("E_TABLE_PARSE {context}: {detail}")]
    TableParse { context: String, detail: String },
    #[error("E_DUPLICATE_KEY: {0}")]
    DuplicateKey(String),
    #[error("E_STATS_MISSING: {stat}")]
    StatsMissing { stat: String },
    #[error("E_IO: {0}")]
    Io(#[from] io::Error),
    #[error("E_INTERNAL: {0}")]
    Internal(String),
}
