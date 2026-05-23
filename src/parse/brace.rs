//! Balanced brace / template extraction utilities leveraging `parse_wiki_text`.

use crate::error::ConvertError;
use crate::parse::default_wiki_configuration;
use parse_wiki_text::{Node, Positioned, WarningMessage};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateSpan {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub raw: String,
    pub line: usize, // 1-based line number of opening '{{'
    pub col: usize,  // 1-based column number of opening '{{'
}

/// Extract top-level `{{...}}` templates with nested depth awareness using `parse_wiki_text`.
pub fn extract_balanced_templates(input: &str) -> Result<Vec<TemplateSpan>, ConvertError> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let output = default_wiki_configuration().parse(input);

    if let Some(warning) = output
        .warnings
        .iter()
        .find(|w| w.message == WarningMessage::MissingEndTagRewinding)
    {
        return Err(ConvertError::UnbalancedBraces {
            name: format!("start@{}", warning.start),
        });
    }

    let mut spans = Vec::new();
    let mut recorded: HashSet<usize> = HashSet::new();
    let index = LineIndex::new(input);
    collect_nodes(
        &output.nodes,
        false,
        input,
        &index,
        &mut spans,
        &mut recorded,
    );
    Ok(spans)
}

fn collect_nodes<'a>(
    nodes: &'a [Node<'a>],
    inside_template: bool,
    source: &'a str,
    index: &LineIndex,
    spans: &mut Vec<TemplateSpan>,
    recorded: &mut HashSet<usize>,
) {
    for node in nodes {
        collect_node(node, inside_template, source, index, spans, recorded);
    }
}

fn collect_node<'a>(
    node: &'a Node<'a>,
    inside_template: bool,
    source: &'a str,
    index: &LineIndex,
    spans: &mut Vec<TemplateSpan>,
    recorded: &mut HashSet<usize>,
) {
    match node {
        Node::Template {
            start,
            end,
            name,
            parameters,
            ..
        } => {
            if !inside_template {
                let raw = source[*start..*end].to_string();
                let name_str = nodes_to_string(name, source);
                let (line, col) = index.line_col(*start);
                if recorded.insert(*start) {
                    spans.push(TemplateSpan {
                        name: name_str,
                        start: *start,
                        end: *end,
                        raw,
                        line,
                        col,
                    });
                }
            }

            for child in name.iter() {
                collect_node(child, true, source, index, spans, recorded);
            }
            for parameter in parameters.iter() {
                if let Some(param_name) = &parameter.name {
                    collect_nodes(param_name, true, source, index, spans, recorded);
                }
                collect_nodes(&parameter.value, true, source, index, spans, recorded);
            }
        }
        Node::Table {
            attributes,
            captions,
            rows,
            ..
        } => {
            collect_nodes(attributes, inside_template, source, index, spans, recorded);
            for caption in captions {
                if let Some(attrs) = &caption.attributes {
                    collect_nodes(attrs, inside_template, source, index, spans, recorded);
                }
                collect_nodes(
                    &caption.content,
                    inside_template,
                    source,
                    index,
                    spans,
                    recorded,
                );
            }
            for row in rows {
                collect_nodes(
                    &row.attributes,
                    inside_template,
                    source,
                    index,
                    spans,
                    recorded,
                );
                for cell in &row.cells {
                    if let Some(attrs) = &cell.attributes {
                        collect_nodes(attrs, inside_template, source, index, spans, recorded);
                    }
                    collect_nodes(
                        &cell.content,
                        inside_template,
                        source,
                        index,
                        spans,
                        recorded,
                    );
                }
            }
        }
        Node::DefinitionList { items, .. } => {
            for item in items {
                collect_nodes(&item.nodes, inside_template, source, index, spans, recorded);
            }
        }
        Node::OrderedList { items, .. } | Node::UnorderedList { items, .. } => {
            for item in items {
                collect_nodes(&item.nodes, inside_template, source, index, spans, recorded);
            }
        }
        Node::Category { ordinal, .. } => {
            collect_nodes(ordinal, inside_template, source, index, spans, recorded);
        }
        Node::ExternalLink { nodes, .. }
        | Node::Heading { nodes, .. }
        | Node::Image { text: nodes, .. }
        | Node::Link { text: nodes, .. }
        | Node::Preformatted { nodes, .. }
        | Node::Tag { nodes, .. } => {
            collect_nodes(nodes, inside_template, source, index, spans, recorded);
        }
        Node::Parameter { name, default, .. } => {
            if !inside_template {
                if let Some(span) = recover_parameter_template(node, source, index) {
                    if recorded.insert(span.start) {
                        spans.push(span);
                    }
                }
            }
            collect_nodes(name, inside_template, source, index, spans, recorded);
            if let Some(default_nodes) = default {
                collect_nodes(
                    default_nodes,
                    inside_template,
                    source,
                    index,
                    spans,
                    recorded,
                );
            }
        }
        Node::Bold { .. }
        | Node::BoldItalic { .. }
        | Node::CharacterEntity { .. }
        | Node::Comment { .. }
        | Node::EndTag { .. }
        | Node::HorizontalDivider { .. }
        | Node::Italic { .. }
        | Node::MagicWord { .. }
        | Node::ParagraphBreak { .. }
        | Node::Redirect { .. }
        | Node::StartTag { .. }
        | Node::Text { .. } => {}
    }
}

fn nodes_to_string(nodes: &[Node<'_>], source: &str) -> String {
    if nodes.is_empty() {
        return String::new();
    }
    let start = nodes.first().map(|n| n.start()).unwrap_or(0);
    let end = nodes.last().map(|n| n.end()).unwrap_or(start);
    source[start..end].trim().to_string()
}

fn recover_parameter_template(
    node: &Node<'_>,
    source: &str,
    index: &LineIndex,
) -> Option<TemplateSpan> {
    let param_start = node.start();
    if param_start == 0 {
        return None;
    }
    let search_end = (param_start + 1).min(source.len());
    let prefix = &source[..search_end];
    let start = prefix.rfind("{{")?;
    let mut depth: i32 = 0;
    let bytes = source.as_bytes();
    let mut i = start;
    while i + 1 <= bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'{' && bytes[i + 1] == b'{' {
            depth += 1;
            i += 2;
            continue;
        }
        if i + 1 < bytes.len() && bytes[i] == b'}' && bytes[i + 1] == b'}' {
            depth -= 1;
            i += 2;
            if depth == 0 {
                let end = i;
                if end <= start + 4 {
                    return None;
                }
                let raw = &source[start..end];
                if raw.len() < 4 {
                    return None;
                }
                let name = raw[2..raw.len() - 2]
                    .split(['|', '\n'])
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                let (line, col) = index.line_col(start);
                return Some(TemplateSpan {
                    name,
                    start,
                    end,
                    raw: raw.to_string(),
                    line,
                    col,
                });
            }
            continue;
        }
        i += 1;
    }
    None
}

struct LineIndex {
    line_starts: Vec<usize>,
}

impl LineIndex {
    fn new(text: &str) -> Self {
        let mut line_starts = Vec::new();
        line_starts.push(0);
        for (idx, ch) in text.char_indices() {
            if ch == '\n' {
                line_starts.push(idx + ch.len_utf8());
            }
        }
        LineIndex { line_starts }
    }

    fn line_col(&self, offset: usize) -> (usize, usize) {
        let idx = match self.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(0) => 0,
            Err(i) => i - 1,
        };

        let line_start = *self.line_starts.get(idx).unwrap_or(&0);
        let line = idx + 1;
        let col = offset.saturating_sub(line_start) + 1;
        (line, col)
    }
}
