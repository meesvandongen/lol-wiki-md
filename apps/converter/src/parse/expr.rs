//! Safe arithmetic expression evaluator for `{{#expr: ...}}` subset.
//!
//! Operator support and precedence mirror MediaWiki's ParserFunctions `#expr`:
//! `or` < `and` < comparison (`= <> != < > <= >=`) < `+ -` < `* / mod` <
//! `^` (left-associative) < unary (`- + not`) < atoms/functions. The trailing
//! `round N` operator is handled by the caller (`split_expr_rounding`), not here.
use crate::error::{ConvertError, Result};

#[derive(Debug, Clone, Copy)]
pub enum ExprNumberFormat {
    Float(u8),
}

pub fn evaluate_expression(expr: &str, fmt: ExprNumberFormat) -> Result<String> {
    let val = evaluate_expression_value(expr)?;
    let ExprNumberFormat::Float(precision) = fmt;
    Ok(format_number(val, precision))
}

/// Evaluate to the raw `f64`, without any formatting/rounding. Callers that
/// need full MediaWiki precision format the result with [`format_sig`].
pub fn evaluate_expression_value(expr: &str) -> Result<f64> {
    let tokens = tokenize(expr)?;
    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
    };
    let val = parser.parse_expr()?;
    if parser.pos != tokens.len() {
        return Err(ConvertError::Expr {
            expr: expr.to_string(),
            detail: "trailing tokens".into(),
        });
    }
    Ok(val)
}

/// Format a value to `sig` significant digits, matching MediaWiki/PHP's default
/// float-to-string precision (14). Rounding to significant digits also absorbs
/// IEEE-754 noise (e.g. `2.666667*300` → `800.0001`, not `800.00009999999997`).
pub fn format_sig(v: f64, sig: i32) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    if !v.is_finite() {
        return format!("{v}");
    }
    let decimals = (sig - 1) - v.abs().log10().floor() as i32;
    let rounded = if (0..=308).contains(&decimals) {
        let factor = 10f64.powi(decimals);
        (v * factor).round() / factor
    } else {
        v
    };
    // Rust's `{}` prints the shortest round-tripping plain decimal (no exponent
    // for the magnitudes seen here), so trailing zeros are already dropped.
    format!("{rounded}")
}

fn format_number(v: f64, p: u8) -> String {
    let s = format!("{:.*}", p as usize, v);
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Cmp {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Num(f64),
    Op(char),
    Cmp(Cmp),
    LParen,
    RParen,
    Comma,
    Ident(String),
}

fn tokenize(input: &str) -> Result<Vec<Tok>> {
    if input.len() > 2000 {
        return Err(ConvertError::Expr {
            expr: input.to_string(),
            detail: "expression too long".into(),
        });
    }
    let mut out = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let bad = |c: char| ConvertError::Expr {
        expr: input.to_string(),
        detail: format!("invalid char '{c}'"),
    };
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        // Comparison operators (possibly two characters).
        match c {
            '=' => {
                out.push(Tok::Cmp(Cmp::Eq));
                i += 1;
                continue;
            }
            '<' => {
                if chars.get(i + 1) == Some(&'=') {
                    out.push(Tok::Cmp(Cmp::Le));
                    i += 2;
                } else if chars.get(i + 1) == Some(&'>') {
                    out.push(Tok::Cmp(Cmp::Ne));
                    i += 2;
                } else {
                    out.push(Tok::Cmp(Cmp::Lt));
                    i += 1;
                }
                continue;
            }
            '>' => {
                if chars.get(i + 1) == Some(&'=') {
                    out.push(Tok::Cmp(Cmp::Ge));
                    i += 2;
                } else {
                    out.push(Tok::Cmp(Cmp::Gt));
                    i += 1;
                }
                continue;
            }
            '!' => {
                if chars.get(i + 1) == Some(&'=') {
                    out.push(Tok::Cmp(Cmp::Ne));
                    i += 2;
                    continue;
                }
                return Err(bad(c));
            }
            _ => {}
        }
        if "()+-*/^,".contains(c) {
            out.push(match c {
                '(' => Tok::LParen,
                ')' => Tok::RParen,
                ',' => Tok::Comma,
                _ => Tok::Op(c),
            });
            i += 1;
            continue;
        }
        if c.is_ascii_digit() || c == '.' {
            let start = i;
            let mut j = i + 1;
            while j < chars.len() && (chars[j].is_ascii_digit() || chars[j] == '.') {
                j += 1;
            }
            // Glued scientific notation: `1.5e2`, `2e-3`. Only consume the `e`
            // suffix when an (optionally signed) digit run follows it, so a bare
            // `e` stays a separate token (Euler's constant / spaced operator).
            if j < chars.len() && (chars[j] == 'e' || chars[j] == 'E') {
                let mut k = j + 1;
                if k < chars.len() && (chars[k] == '+' || chars[k] == '-') {
                    k += 1;
                }
                if k < chars.len() && chars[k].is_ascii_digit() {
                    while k < chars.len() && chars[k].is_ascii_digit() {
                        k += 1;
                    }
                    j = k;
                }
            }
            let lit = chars[start..j].iter().collect::<String>();
            let num: f64 = lit.parse().map_err(|e| ConvertError::Expr {
                expr: lit.clone(),
                detail: format!("bad number: {e}"),
            })?;
            out.push(Tok::Num(num));
            i = j;
            continue;
        }
        if c.is_ascii_alphabetic() {
            let start = i;
            let mut j = i + 1;
            while j < chars.len() && (chars[j].is_ascii_alphanumeric() || chars[j] == '_') {
                j += 1;
            }
            out.push(Tok::Ident(chars[start..j].iter().collect()));
            i = j;
            continue;
        }
        return Err(bad(c));
    }
    Ok(out)
}

/// True if an `Ident` token equals `kw` (case-insensitive) — used for the
/// word operators `and`, `or`, `not`, `mod`.
fn is_kw(tok: Option<&Tok>, kw: &str) -> bool {
    matches!(tok, Some(Tok::Ident(s)) if s.eq_ignore_ascii_case(kw))
}

struct Parser<'a> {
    tokens: &'a [Tok],
    pos: usize,
}
impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&'a Tok> {
        self.tokens.get(self.pos)
    }
    fn bump(&mut self) -> Option<&'a Tok> {
        let t = self.tokens.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }
    fn parse_expr(&mut self) -> Result<f64> {
        self.parse_or()
    }
    fn parse_or(&mut self) -> Result<f64> {
        let mut v = self.parse_and()?;
        while is_kw(self.peek(), "or") {
            self.bump();
            let rhs = self.parse_and()?;
            v = if v != 0.0 || rhs != 0.0 { 1.0 } else { 0.0 };
        }
        Ok(v)
    }
    fn parse_and(&mut self) -> Result<f64> {
        let mut v = self.parse_cmp()?;
        while is_kw(self.peek(), "and") {
            self.bump();
            let rhs = self.parse_cmp()?;
            v = if v != 0.0 && rhs != 0.0 { 1.0 } else { 0.0 };
        }
        Ok(v)
    }
    fn parse_cmp(&mut self) -> Result<f64> {
        let mut v = self.parse_add_sub()?;
        while let Some(Tok::Cmp(op)) = self.peek() {
            let op = op.clone();
            self.bump();
            let rhs = self.parse_add_sub()?;
            let b = match op {
                Cmp::Eq => v == rhs,
                Cmp::Ne => v != rhs,
                Cmp::Lt => v < rhs,
                Cmp::Gt => v > rhs,
                Cmp::Le => v <= rhs,
                Cmp::Ge => v >= rhs,
            };
            v = if b { 1.0 } else { 0.0 };
        }
        Ok(v)
    }
    fn parse_add_sub(&mut self) -> Result<f64> {
        let mut v = self.parse_mul_div()?;
        loop {
            match self.peek() {
                Some(Tok::Op('+')) => {
                    self.bump();
                    v += self.parse_mul_div()?;
                }
                Some(Tok::Op('-')) => {
                    self.bump();
                    v -= self.parse_mul_div()?;
                }
                _ => break,
            }
        }
        Ok(v)
    }
    fn parse_mul_div(&mut self) -> Result<f64> {
        let mut v = self.parse_pow()?;
        loop {
            match self.peek() {
                Some(Tok::Op('*')) => {
                    self.bump();
                    v *= self.parse_pow()?;
                }
                Some(Tok::Op('/')) => {
                    self.bump();
                    let rhs = self.parse_pow()?;
                    if rhs == 0.0 {
                        return Err(ConvertError::Expr {
                            expr: "/ division".into(),
                            detail: "division by zero".into(),
                        });
                    }
                    v /= rhs;
                }
                // MediaWiki `mod` is integer remainder: (int)a % (int)b.
                t if is_kw(t, "mod") => {
                    self.bump();
                    let rhs = self.parse_pow()?;
                    let ri = rhs.trunc() as i64;
                    if ri == 0 {
                        return Err(ConvertError::Expr {
                            expr: "mod".into(),
                            detail: "division by zero".into(),
                        });
                    }
                    v = (v.trunc() as i64 % ri) as f64;
                }
                _ => break,
            }
        }
        Ok(v)
    }
    fn parse_pow(&mut self) -> Result<f64> {
        // Left-associative exponentiation, matching MediaWiki: a^b^c = (a^b)^c.
        let mut v = self.parse_unary()?;
        while let Some(Tok::Op('^')) = self.peek() {
            self.bump();
            let rhs = self.parse_unary()?;
            v = v.powf(rhs);
        }
        Ok(v)
    }
    fn parse_unary(&mut self) -> Result<f64> {
        match self.peek() {
            Some(Tok::Op('-')) => {
                self.bump();
                Ok(-self.parse_unary()?)
            }
            Some(Tok::Op('+')) => {
                self.bump();
                self.parse_unary()
            }
            t if is_kw(t, "not") => {
                self.bump();
                let v = self.parse_unary()?;
                Ok(if v == 0.0 { 1.0 } else { 0.0 })
            }
            _ => {
                let base = self.parse_atom()?;
                self.maybe_e_notation(base)
            }
        }
    }
    /// Scientific notation: `a e b` means `a × 10^b` (MediaWiki). `e` is only an
    /// operator when an operand follows it; a bare `e` is Euler's constant.
    fn maybe_e_notation(&mut self, base: f64) -> Result<f64> {
        if is_kw(self.peek(), "e")
            && matches!(
                self.tokens.get(self.pos + 1),
                Some(Tok::Num(_) | Tok::Op('-') | Tok::Op('+') | Tok::LParen)
            )
        {
            self.bump(); // consume `e`
            let exp = self.parse_unary()?;
            return Ok(base * 10f64.powf(exp));
        }
        Ok(base)
    }
    fn parse_atom(&mut self) -> Result<f64> {
        match self.bump() {
            Some(Tok::Num(n)) => Ok(*n),
            Some(Tok::LParen) => {
                let v = self.parse_expr()?;
                match self.bump() {
                    Some(Tok::RParen) => Ok(v),
                    _ => Err(ConvertError::Expr {
                        expr: "()".into(),
                        detail: "missing )".into(),
                    }),
                }
            }
            Some(Tok::Ident(id)) => self.parse_ident(id.clone()),
            other => Err(ConvertError::Expr {
                expr: "atom".into(),
                detail: format!("unexpected token {:?}", other),
            }),
        }
    }
    fn parse_ident(&mut self, id: String) -> Result<f64> {
        let lname = id.to_lowercase();
        // A function call if followed by `(`; otherwise a named constant or a
        // unary prefix function (MediaWiki allows `sqrt 4`, `ln e`, etc.).
        if matches!(self.peek(), Some(Tok::LParen)) {
            self.bump();
            let mut args = Vec::new();
            if let Some(Tok::RParen) = self.peek() {
                self.bump();
            } else {
                loop {
                    args.push(self.parse_expr()?);
                    match self.bump() {
                        Some(Tok::RParen) => break,
                        Some(Tok::Comma) => continue,
                        other => {
                            return Err(ConvertError::Expr {
                                expr: id.clone(),
                                detail: format!("expected , or ) got {:?}", other),
                            })
                        }
                    }
                }
            }
            return apply_func(&lname, &args);
        }
        match lname.as_str() {
            "pi" => Ok(std::f64::consts::PI),
            "e" => Ok(std::f64::consts::E),
            // Unary prefix functions: `sqrt 4`, `abs -2`, `ceil 1.1`, ...
            "sqrt" | "trunc" | "ln" | "exp" | "abs" | "ceil" | "floor" | "round" | "sin"
            | "cos" | "tan" => {
                let arg = self.parse_unary()?;
                apply_func(&lname, &[arg])
            }
            _ => Err(ConvertError::Expr {
                expr: id.clone(),
                detail: format!("unknown ident {id}"),
            }),
        }
    }
}

fn apply_func(name: &str, args: &[f64]) -> Result<f64> {
    let f = |n: &str| ConvertError::Expr {
        expr: n.into(),
        detail: "bad args".into(),
    };
    let unary = |a: &[f64]| -> Result<f64> {
        if a.len() != 1 {
            Err(f(name))
        } else {
            Ok(a[0])
        }
    };
    match name {
        "round" => Ok(unary(args)?.round()),
        "floor" => Ok(unary(args)?.floor()),
        "ceil" => Ok(unary(args)?.ceil()),
        "abs" => Ok(unary(args)?.abs()),
        "trunc" => Ok(unary(args)?.trunc()),
        "sqrt" => Ok(unary(args)?.sqrt()),
        "ln" => Ok(unary(args)?.ln()),
        "exp" => Ok(unary(args)?.exp()),
        "sin" => Ok(unary(args)?.sin()),
        "cos" => Ok(unary(args)?.cos()),
        "tan" => Ok(unary(args)?.tan()),
        "min" => {
            if args.is_empty() {
                return Err(f(name));
            }
            Ok(args.iter().copied().fold(f64::INFINITY, f64::min))
        }
        "max" => {
            if args.is_empty() {
                return Err(f(name));
            }
            Ok(args.iter().copied().fold(f64::NEG_INFINITY, f64::max))
        }
        _ => Err(ConvertError::Expr {
            expr: name.into(),
            detail: format!("unknown function {name}"),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_arith() {
        assert_eq!(
            evaluate_expression("1+2*3", ExprNumberFormat::Float(2)).unwrap(),
            "7"
        );
    }
    #[test]
    fn precedence_paren() {
        assert_eq!(
            evaluate_expression("(1+2)*3", ExprNumberFormat::Float(2)).unwrap(),
            "9"
        );
    }
    #[test]
    fn pow_is_left_associative() {
        // MediaWiki: 2^3^2 = (2^3)^2 = 64.
        assert_eq!(
            evaluate_expression("2^3^2", ExprNumberFormat::Float(2)).unwrap(),
            "64"
        );
    }
    #[test]
    fn modulo_is_integer() {
        assert_eq!(
            evaluate_expression("10 mod 3", ExprNumberFormat::Float(2)).unwrap(),
            "1"
        );
        assert_eq!(
            evaluate_expression("5 mod 3", ExprNumberFormat::Float(2)).unwrap(),
            "2"
        );
    }
    #[test]
    fn comparison_and_boolean() {
        assert_eq!(
            evaluate_expression("5 > 3", ExprNumberFormat::Float(2)).unwrap(),
            "1"
        );
        assert_eq!(
            evaluate_expression("5 < 3", ExprNumberFormat::Float(2)).unwrap(),
            "0"
        );
        assert_eq!(
            evaluate_expression("1 and 0", ExprNumberFormat::Float(2)).unwrap(),
            "0"
        );
        assert_eq!(
            evaluate_expression("1 or 0", ExprNumberFormat::Float(2)).unwrap(),
            "1"
        );
        assert_eq!(
            evaluate_expression("not 0", ExprNumberFormat::Float(2)).unwrap(),
            "1"
        );
    }
    #[test]
    fn e_notation() {
        assert_eq!(
            evaluate_expression("2 e 3", ExprNumberFormat::Float(2)).unwrap(),
            "2000"
        );
        assert_eq!(
            evaluate_expression("1.5e2", ExprNumberFormat::Float(2)).unwrap(),
            "150"
        );
    }
    #[test]
    fn prefix_functions() {
        assert_eq!(
            evaluate_expression("sqrt 16", ExprNumberFormat::Float(2)).unwrap(),
            "4"
        );
        assert_eq!(
            evaluate_expression("trunc 7.9", ExprNumberFormat::Float(2)).unwrap(),
            "7"
        );
    }
    #[test]
    fn full_precision_when_unrounded() {
        assert_eq!(
            evaluate_expression("700/2200", ExprNumberFormat::Float(14)).unwrap(),
            "0.31818181818182"
        );
    }
    #[test]
    fn funcs() {
        assert_eq!(
            evaluate_expression("min(5, 2, 3)", ExprNumberFormat::Float(0)).unwrap(),
            "2"
        );
    }
}
