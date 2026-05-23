pub mod ability;
pub mod brace;
pub mod expr;
pub mod lua;
pub mod tables;
pub mod templates;

use once_cell::sync::Lazy;
use parse_wiki_text::Configuration;

pub use ability::parse_ability_template;
pub use brace::{extract_balanced_templates, TemplateSpan};
pub use expr::{evaluate_expression, ExprNumberFormat};
pub use tables::wikitext_table_to_markdown;
pub use templates::{ExpansionResult, TemplateInvocation, TemplateRegistry};

pub(crate) fn default_wiki_configuration() -> &'static Configuration {
    static CONFIG: Lazy<Configuration> = Lazy::new(Configuration::default);
    &CONFIG
}
