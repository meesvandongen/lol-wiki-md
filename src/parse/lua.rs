//! Narrow Lua table parser implementation for ChampionData / ItemData subsets.
//! Supports extracting a single keyed top-level table and returning flat string values for primitive fields.
use std::collections::HashMap;
use crate::error::{Result, ConvertError};

#[derive(Debug, PartialEq)]
enum LTok { LBrace, RBrace, LBracket, RBracket, Eq, Comma, String(String), Number(String), Ident(String) }

pub fn parse_champion_data(content: &str, champion: &str) -> Result<HashMap<String,String>> {
    // Locate top-level entry: ["Champion"] = { ... }
    let needle1 = format!("[\"{champion}\"]");
    let needle2 = format!("['{champion}']");
    let start = content.find(&needle1).or_else(|| content.find(&needle2)).ok_or_else(|| ConvertError::ChampionNotFound(champion.to_string()))?;
    // Find first '{' after '='
    let eq_pos = content[start..].find('=').ok_or_else(|| ConvertError::LuaParse { detail: "missing = after key".into() })? + start;
    let brace_pos = content[eq_pos..].find('{').ok_or_else(|| ConvertError::LuaParse { detail: "missing { after =".into() })? + eq_pos;
    let (table_src, _) = slice_balanced_braces(content, brace_pos)?;
    let tokens = tokenize(table_src)?; // includes outer braces
    let mut parser = Parser { tokens: &tokens, pos: 0 };
    let value = parser.parse_table()?;
    if parser.pos != tokens.len() { return Err(ConvertError::LuaParse { detail: "trailing tokens".into() }); }
    Ok(value)
}

fn slice_balanced_braces(s: &str, open_idx: usize) -> Result<(&str, usize)> {
    let mut depth = 0usize; let bytes = s.as_bytes();
    for i in open_idx..bytes.len() { match bytes[i] as char { '{' => depth+=1, '}' => { depth-=1; if depth==0 { return Ok((&s[open_idx..=i], i)); } }, _=>{} } }
    Err(ConvertError::LuaParse { detail: "unbalanced braces".into() })
}

fn tokenize(src: &str) -> Result<Vec<LTok>> {
    let mut out = Vec::new(); let chars: Vec<char> = src.chars().collect(); let mut i=0;
    while i < chars.len() { let c = chars[i];
        if c.is_whitespace() { i+=1; continue; }
        // Handle comments and unary minus
        if c == '-' {
            // Line or block comments start with --
            if i+1 < chars.len() && chars[i+1] == '-' {
                // Block comment --[[ ... ]]
                if i+3 < chars.len() && chars[i+2] == '[' && chars[i+3] == '[' {
                    i += 4; // skip --[[
                    // consume until ]]
                    while i+1 < chars.len() && !(chars[i] == ']' && chars[i+1] == ']') { i += 1; }
                    i = (i+2).min(chars.len());
                    continue;
                }
                // Line comment -- ... (to end of line)
                i += 2; // skip --
                while i < chars.len() && chars[i] != '\n' { i += 1; }
                continue;
            }
            // Unary minus for numbers
            if i+1 < chars.len() && (chars[i+1].is_ascii_digit()) {
                let start = i; let mut j = i+1;
                while j < chars.len() && (chars[j].is_ascii_digit() || chars[j] == '.') { j += 1; }
                out.push(LTok::Number(chars[start..j].iter().collect()));
                i = j; continue;
            }
            return Err(ConvertError::LuaParse { detail: format!("unexpected char {c}") });
        }
        match c { '{' => { out.push(LTok::LBrace); i+=1; continue; }, '}' => { out.push(LTok::RBrace); i+=1; continue; }, '[' => { out.push(LTok::LBracket); i+=1; continue; }, ']' => { out.push(LTok::RBracket); i+=1; continue; }, '=' => { out.push(LTok::Eq); i+=1; continue; }, ',' => { out.push(LTok::Comma); i+=1; continue; }, '\''| '"' => { let quote=c; let mut j=i+1; let mut buf=String::new(); while j<chars.len() { let d=chars[j]; if d==quote { break; } buf.push(d); j+=1; } if j==chars.len() { return Err(ConvertError::LuaParse { detail: "unterminated string".into() }); } out.push(LTok::String(buf)); i=j+1; continue; }, _=>{} }
        if c.is_ascii_digit() { let start=i; let mut j=i+1; while j<chars.len() && (chars[j].is_ascii_digit()||chars[j]=='.') { j+=1; } out.push(LTok::Number(chars[start..j].iter().collect())); i=j; continue; }
        if c.is_ascii_alphabetic() || c=='_' { let start=i; let mut j=i+1; while j<chars.len() && (chars[j].is_ascii_alphanumeric()||chars[j]=='_') { j+=1; } out.push(LTok::Ident(chars[start..j].iter().collect())); i=j; continue; }
        return Err(ConvertError::LuaParse { detail: format!("unexpected char {c}") });
    }
    Ok(out)
}

struct Parser<'a> { tokens: &'a [LTok], pos: usize }
impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&'a LTok> { self.tokens.get(self.pos) }
    fn bump(&mut self) -> Option<&'a LTok> { let t=self.tokens.get(self.pos); if t.is_some() { self.pos+=1; } t }
    fn expect(&mut self, expect: LTok) -> Result<()> where LTok: PartialEq { let t=self.bump(); if t==Some(&expect) { Ok(()) } else { Err(ConvertError::LuaParse { detail: format!("expected {:?} got {:?}", expect, t) }) } }
    fn parse_table(&mut self) -> Result<HashMap<String,String>> { self.expect(LTok::LBrace)?; let mut map=HashMap::new(); loop {
            match self.peek() {
                Some(LTok::RBrace) => { self.bump(); break; },
                Some(_) => { let (k,v)=self.parse_kv()?; if map.insert(k.clone(), v).is_some() { return Err(ConvertError::DuplicateKey(k)); } match self.peek() { Some(LTok::Comma) => { self.bump(); continue; }, Some(LTok::RBrace)=>{}, other=> return Err(ConvertError::LuaParse { detail: format!("expected , or }} got {:?}", other) }) } },
                None => return Err(ConvertError::LuaParse { detail: "eof in table".into() })
            }
        }
        Ok(map)
    }
    fn parse_kv(&mut self) -> Result<(String,String)> { // key can be ["Key"] or Ident
        let key = match self.bump() { Some(LTok::LBracket) => { let inner = match self.bump() { Some(LTok::String(s)) => s.clone(), _ => return Err(ConvertError::LuaParse { detail: "expected string key".into() }) }; match self.bump() { Some(LTok::RBracket)=>{}, _=> return Err(ConvertError::LuaParse { detail: "missing ]".into() }) }; inner }, Some(LTok::Ident(id)) => id.clone(), other => return Err(ConvertError::LuaParse { detail: format!("invalid key token {:?}", other) }) };
        match self.bump() { Some(LTok::Eq)=>{}, _=> return Err(ConvertError::LuaParse { detail: "missing =".into() }) };
        let val = self.parse_value()?;
        Ok((key,val))
    }
    fn parse_value(&mut self) -> Result<String> { match self.bump() { Some(LTok::String(s)) => Ok(s.clone()), Some(LTok::Number(n)) => Ok(n.clone()), Some(LTok::Ident(id)) => Ok(id.clone()), Some(LTok::LBrace) => { // nested table -> capture until balanced
            let start = self.pos-1; // position of '{'
            let mut depth=1; while depth>0 { match self.bump() { Some(LTok::LBrace)=> depth+=1, Some(LTok::RBrace)=> depth-=1, Some(_) => {}, None => return Err(ConvertError::LuaParse { detail: "unterminated nested table".into() }) } }
            // Reconstruct substring (simplified representation)
            let subslice = &self.tokens[start..self.pos];
            Ok(format!("<table:{} tokens>", subslice.len()))
        }, other => Err(ConvertError::LuaParse { detail: format!("invalid value token {:?}", other) }) }
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
}
