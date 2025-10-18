//! Narrow Lua table parser implementation for ChampionData / ItemData subsets.
//! Supports extracting a single keyed top-level table and returning flat string values for primitive fields.
use crate::error::{ConvertError, Result};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum LTok {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Eq,
    Comma,
    String(String),
    Number(String),
    Ident(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LuaValue {
    Nil,
    Bool(bool),
    Number(String),
    String(String),
    Table(HashMap<String, LuaValue>),
    Array(Vec<LuaValue>),
}

pub fn parse_champion_entry(content: &str, champion: &str) -> Result<HashMap<String, LuaValue>> {
    let root = parse_module_root(content)?;
    let entry = root
        .get(champion)
        .or_else(|| {
            root.iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(champion))
                .map(|(_, v)| v)
        })
        .ok_or_else(|| ConvertError::ChampionNotFound(champion.to_string()))?;
    match entry {
        LuaValue::Table(map) => Ok(map.clone()),
        other => Err(ConvertError::LuaParse {
            detail: format!("expected table for champion entry, found {other:?}"),
        }),
    }
}

pub fn parse_champion_data(content: &str, champion: &str) -> Result<HashMap<String, String>> {
    let table = parse_champion_entry(content, champion)?;
    Ok(table_to_string_map(&table))
}

pub fn parse_item_data(content: &str, item: &str) -> Result<HashMap<String, LuaValue>> {
    let root = parse_module_root(content)?;
    let entry = root
        .get(item)
        .or_else(|| {
            root.iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(item))
                .map(|(_, v)| v)
        })
        .ok_or_else(|| ConvertError::ItemNotFound(item.to_string()))?;
    match entry {
        LuaValue::Table(map) => Ok(map.clone()),
        other => Err(ConvertError::LuaParse {
            detail: format!("expected table for item entry, found {other:?}"),
        }),
    }
}

pub fn parse_item_module(content: &str) -> Result<HashMap<String, HashMap<String, LuaValue>>> {
    let root = parse_module_root(content)?;
    let mut out: HashMap<String, HashMap<String, LuaValue>> = HashMap::new();
    for (name, value) in root {
        if let LuaValue::Table(map) = value {
            out.insert(name, map);
        }
    }
    Ok(out)
}

fn parse_module_root(content: &str) -> Result<HashMap<String, LuaValue>> {
    let ret_idx = content
        .find("return")
        .ok_or_else(|| ConvertError::LuaParse {
            detail: "missing return in module".into(),
        })?;
    let open_rel = content[ret_idx..]
        .find('{')
        .ok_or_else(|| ConvertError::LuaParse {
            detail: "missing { after return".into(),
        })?;
    let open_idx = ret_idx + open_rel;
    let (table_src, _) = slice_balanced_braces(content, open_idx)?;
    let tokens = tokenize(table_src)?;
    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
    };
    let value = parser.parse_table_value()?;
    if parser.pos != tokens.len() {
        return Err(ConvertError::LuaParse {
            detail: "trailing tokens after module parse".into(),
        });
    }
    match value {
        LuaValue::Table(map) => Ok(map),
        LuaValue::Array(arr) => {
            let mut map = HashMap::new();
            for (idx, val) in arr.into_iter().enumerate() {
                map.insert((idx + 1).to_string(), val);
            }
            Ok(map)
        }
        other => Err(ConvertError::LuaParse {
            detail: format!("module root must be table, got {other:?}"),
        }),
    }
}

fn table_to_string_map(table: &HashMap<String, LuaValue>) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for (k, v) in table {
        if let Some(s) = lua_value_to_string(v) {
            out.insert(k.clone(), s);
        }
    }
    out
}

pub fn lua_value_to_string(value: &LuaValue) -> Option<String> {
    match value {
        LuaValue::Nil => None,
        LuaValue::Bool(b) => Some(b.to_string()),
        LuaValue::Number(s) => Some(s.clone()),
        LuaValue::String(s) => Some(s.clone()),
        LuaValue::Table(_) | LuaValue::Array(_) => None,
    }
}

pub fn lua_value_to_string_vec(value: &LuaValue) -> Option<Vec<String>> {
    match value {
        LuaValue::Array(items) => {
            let mut out = Vec::new();
            for v in items {
                if let Some(s) = lua_value_to_string(v) {
                    if !s.is_empty() {
                        out.push(s);
                    }
                }
            }
            Some(out)
        }
        LuaValue::Table(map) => {
            // Treat numeric keys as array fallback.
            let mut indexed: Vec<(usize, String)> = Vec::new();
            for (k, v) in map {
                if let Ok(idx) = k.parse::<usize>() {
                    if let Some(val) = lua_value_to_string(v) {
                        indexed.push((idx, val));
                    }
                }
            }
            if indexed.is_empty() {
                None
            } else {
                indexed.sort_by_key(|(idx, _)| *idx);
                Some(indexed.into_iter().map(|(_, v)| v).collect())
            }
        }
        _ => None,
    }
}

fn slice_balanced_braces(s: &str, open_idx: usize) -> Result<(&str, usize)> {
    let mut depth = 0usize;
    let bytes = s.as_bytes();
    for i in open_idx..bytes.len() {
        match bytes[i] as char {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok((&s[open_idx..=i], i));
                }
            }
            _ => {}
        }
    }
    Err(ConvertError::LuaParse {
        detail: "unbalanced braces".into(),
    })
}

fn is_numeric_expr_char(c: char) -> bool {
    c.is_ascii_digit() || matches!(c, '.' | '+' | '-' | '*' | '/' | '^' | '(' | ')')
}

fn tokenize(src: &str) -> Result<Vec<LTok>> {
    let mut out = Vec::new();
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        // Handle comments and unary minus
        if c == '-' {
            // Line or block comments start with --
            if i + 1 < chars.len() && chars[i + 1] == '-' {
                // Block comment --[[ ... ]]
                if i + 3 < chars.len() && chars[i + 2] == '[' && chars[i + 3] == '[' {
                    i += 4; // skip --[[
                            // consume until ]]
                    while i + 1 < chars.len() && !(chars[i] == ']' && chars[i + 1] == ']') {
                        i += 1;
                    }
                    i = (i + 2).min(chars.len());
                    continue;
                }
                // Line comment -- ... (to end of line)
                i += 2; // skip --
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            // Unary minus for numbers
            if i + 1 < chars.len() && (chars[i + 1].is_ascii_digit()) {
                let start = i;
                let mut j = i + 1;
                while j < chars.len() && is_numeric_expr_char(chars[j]) {
                    j += 1;
                }
                out.push(LTok::Number(chars[start..j].iter().collect()));
                i = j;
                continue;
            }
            return Err(ConvertError::LuaParse {
                detail: format!("unexpected char {c}"),
            });
        }
        if c == '+' {
            if i + 1 < chars.len() && chars[i + 1].is_ascii_digit() {
                let start = i;
                let mut j = i + 1;
                while j < chars.len() && is_numeric_expr_char(chars[j]) {
                    j += 1;
                }
                out.push(LTok::Number(chars[start..j].iter().collect()));
                i = j;
                continue;
            }
            return Err(ConvertError::LuaParse {
                detail: format!("unexpected char {c}"),
            });
        }
        match c {
            '{' => {
                out.push(LTok::LBrace);
                i += 1;
                continue;
            }
            '}' => {
                out.push(LTok::RBrace);
                i += 1;
                continue;
            }
            '[' => {
                out.push(LTok::LBracket);
                i += 1;
                continue;
            }
            ']' => {
                out.push(LTok::RBracket);
                i += 1;
                continue;
            }
            '=' => {
                out.push(LTok::Eq);
                i += 1;
                continue;
            }
            ',' => {
                out.push(LTok::Comma);
                i += 1;
                continue;
            }
            '\'' | '"' => {
                let quote = c;
                let mut j = i + 1;
                let mut buf = String::new();
                while j < chars.len() {
                    let d = chars[j];
                    if d == quote {
                        break;
                    }
                    buf.push(d);
                    j += 1;
                }
                if j == chars.len() {
                    return Err(ConvertError::LuaParse {
                        detail: "unterminated string".into(),
                    });
                }
                out.push(LTok::String(buf));
                i = j + 1;
                continue;
            }
            _ => {}
        }
        if c.is_ascii_digit() {
            let start = i;
            let mut j = i + 1;
            while j < chars.len() && is_numeric_expr_char(chars[j]) {
                j += 1;
            }
            out.push(LTok::Number(chars[start..j].iter().collect()));
            i = j;
            continue;
        }
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            let mut j = i + 1;
            while j < chars.len() && (chars[j].is_ascii_alphanumeric() || chars[j] == '_') {
                j += 1;
            }
            out.push(LTok::Ident(chars[start..j].iter().collect()));
            i = j;
            continue;
        }
        return Err(ConvertError::LuaParse {
            detail: format!("unexpected char {c}"),
        });
    }
    Ok(out)
}

struct Parser<'a> {
    tokens: &'a [LTok],
    pos: usize,
}
impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&'a LTok> {
        self.tokens.get(self.pos)
    }
    fn peek_n(&self, offset: usize) -> Option<&'a LTok> {
        self.tokens.get(self.pos + offset)
    }
    fn bump(&mut self) -> Option<&'a LTok> {
        let t = self.tokens.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }
    fn expect(&mut self, expect: LTok) -> Result<()>
    where
        LTok: PartialEq,
    {
        let t = self.bump();
        if t == Some(&expect) {
            Ok(())
        } else {
            Err(ConvertError::LuaParse {
                detail: format!("expected {:?} got {:?}", expect, t),
            })
        }
    }

    fn parse_table_value(&mut self) -> Result<LuaValue> {
        self.expect(LTok::LBrace)?;
        let mut map: HashMap<String, LuaValue> = HashMap::new();
        let mut array: Vec<LuaValue> = Vec::new();
        let mut saw_keyed = false;
        let mut saw_array = false;
        loop {
            match self.peek() {
                Some(LTok::RBrace) => {
                    self.bump();
                    break;
                }
                None => {
                    return Err(ConvertError::LuaParse {
                        detail: "eof in table".into(),
                    })
                }
                _ => {
                    if self.is_next_keyed()? {
                        let (key, value) = self.parse_keyed_entry()?;
                        map.insert(key, value);
                        saw_keyed = true;
                    } else {
                        let value = self.parse_value()?;
                        array.push(value);
                        saw_array = true;
                    }
                    match self.peek() {
                        Some(LTok::Comma) => {
                            self.bump();
                        }
                        Some(LTok::RBrace) => {}
                        None => {}
                        other => {
                            return Err(ConvertError::LuaParse {
                                detail: format!("expected , or }} got {:?}", other),
                            });
                        }
                    }
                }
            }
        }
        if saw_keyed {
            if saw_array {
                for (idx, val) in array.into_iter().enumerate() {
                    map.entry((idx + 1).to_string()).or_insert(val);
                }
            }
            Ok(LuaValue::Table(map))
        } else if saw_array {
            Ok(LuaValue::Array(array))
        } else {
            Ok(LuaValue::Table(map))
        }
    }

    fn is_next_keyed(&self) -> Result<bool> {
        match self.peek() {
            Some(LTok::LBracket) => Ok(true),
            Some(LTok::Ident(_)) => Ok(matches!(self.peek_n(1), Some(LTok::Eq))),
            Some(LTok::String(_)) => Ok(matches!(self.peek_n(1), Some(LTok::Eq))),
            Some(LTok::Number(_)) => Ok(matches!(self.peek_n(1), Some(LTok::Eq))),
            _ => Ok(false),
        }
    }

    fn parse_keyed_entry(&mut self) -> Result<(String, LuaValue)> {
        let key = match self.bump() {
            Some(LTok::LBracket) => {
                let inner = match self.bump() {
                    Some(LTok::String(s)) => s.clone(),
                    Some(LTok::Number(n)) => n.clone(),
                    Some(LTok::Ident(id)) => id.clone(),
                    other => {
                        return Err(ConvertError::LuaParse {
                            detail: format!("expected key inside [], got {:?}", other),
                        })
                    }
                };
                self.expect(LTok::RBracket)?;
                inner
            }
            Some(LTok::String(s)) => s.clone(),
            Some(LTok::Number(n)) => n.clone(),
            Some(LTok::Ident(id)) => id.clone(),
            other => {
                return Err(ConvertError::LuaParse {
                    detail: format!("invalid key token {:?}", other),
                })
            }
        };
        self.expect(LTok::Eq)?;
        let value = self.parse_value()?;
        Ok((key, value))
    }

    fn parse_value(&mut self) -> Result<LuaValue> {
        match self.peek() {
            Some(LTok::LBrace) => self.parse_table_value(),
            Some(LTok::String(_)) => {
                if let Some(LTok::String(s)) = self.bump() {
                    Ok(LuaValue::String(s.clone()))
                } else {
                    unreachable!()
                }
            }
            Some(LTok::Number(_)) => {
                if let Some(LTok::Number(n)) = self.bump() {
                    Ok(LuaValue::Number(n.clone()))
                } else {
                    unreachable!()
                }
            }
            Some(LTok::Ident(_)) => {
                if let Some(LTok::Ident(id)) = self.bump() {
                    match id.as_str() {
                        "true" => Ok(LuaValue::Bool(true)),
                        "false" => Ok(LuaValue::Bool(false)),
                        "nil" => Ok(LuaValue::Nil),
                        _ => Ok(LuaValue::String(id.clone())),
                    }
                } else {
                    unreachable!()
                }
            }
            Some(tok) => Err(ConvertError::LuaParse {
                detail: format!("unexpected token in value: {:?}", tok),
            }),
            None => Err(ConvertError::LuaParse {
                detail: "unexpected eof in value".into(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_champion_table() {
        let lua = r#"return {
  ["Aatrox"] = { hp = 650, hpGrowth = 104, resource = "Blood Well" },
  ["Ahri"] = { hp = 590, hpGrowth = 99 }
}"#;
        let map = parse_champion_data(lua, "Aatrox").unwrap();
        assert_eq!(map.get("hp").unwrap(), "650");
        assert_eq!(map.get("hpGrowth").unwrap(), "104");
        assert_eq!(map.get("resource").unwrap(), "Blood Well");
    }

    #[test]
    fn parse_item_table_with_arrays_and_nested() {
        let lua = r#"return {
  ["Test Blade"] = {
    tier = 3,
    type = {"Legendary", "Weapon"},
    stats = { ap = 80, ah = 20 },
    effects = {
      pass = { name = "Edge", unique = true, description = "Grants {{ap|40}}." },
      act = { name = "Slash", cd = "60", range = "Global" }
    },
    recipe = {"B. F. Sword", "Pickaxe"},
    buy = 3300,
    sellratio = 0.6,
  }
}"#;
        let item = parse_item_data(lua, "Test Blade").unwrap();
        assert!(matches!(item.get("tier"), Some(LuaValue::Number(n)) if n == "3"));
        let types = lua_value_to_string_vec(item.get("type").unwrap()).unwrap();
        assert_eq!(types, vec!["Legendary".to_string(), "Weapon".to_string()]);
        let stats = item.get("stats").unwrap();
        if let LuaValue::Table(stats_map) = stats {
            assert_eq!(
                lua_value_to_string(stats_map.get("ap").unwrap()).unwrap(),
                "80"
            );
            assert_eq!(
                lua_value_to_string(stats_map.get("ah").unwrap()).unwrap(),
                "20"
            );
        } else {
            panic!("expected stats table");
        }
        let effects = item.get("effects").unwrap();
        if let LuaValue::Table(eff_map) = effects {
            assert!(eff_map.contains_key("pass"));
            assert!(eff_map.contains_key("act"));
        } else {
            panic!("expected effects table");
        }
    }

    #[test]
    fn duplicate_keys_last_wins() {
        let lua = r#"return {
  ["Tester"] = {
    foo = "first",
    foo = "second",
  }
}"#;
        let map = parse_champion_data(lua, "Tester").unwrap();
        assert_eq!(map.get("foo"), Some(&"second".to_string()));
    }
}
