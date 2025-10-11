//! Safe arithmetic expression evaluator for `{{#expr: ...}}` subset.
use crate::error::{ConvertError, Result};

#[derive(Debug, Clone, Copy)]
pub enum ExprNumberFormat {
    Float(u8),
}

pub fn evaluate_expression(expr: &str, fmt: ExprNumberFormat) -> Result<String> {
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
    let precision = match fmt {
        ExprNumberFormat::Float(p) => p,
    };
    Ok(format_number(val, precision))
}

fn format_number(v: f64, p: u8) -> String {
    let s = format!("{:.*}", p as usize, v);
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Num(f64),
    Op(char),
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
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
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
        return Err(ConvertError::Expr {
            expr: input.to_string(),
            detail: format!("invalid char '{c}'"),
        });
    }
    Ok(out)
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
        self.parse_add_sub()
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
                _ => break,
            }
        }
        Ok(v)
    }
    fn parse_pow(&mut self) -> Result<f64> {
        // Right-associative exponentiation: parse a^b^c as a^(b^c)
        let base = self.parse_unary()?;
        self.parse_pow_rhs(base)
    }
    fn parse_pow_rhs(&mut self, lhs: f64) -> Result<f64> {
        if let Some(Tok::Op('^')) = self.peek() {
            self.bump();
            let rhs_unary = self.parse_unary()?;
            // If another ^ follows, recurse to ensure right-associativity.
            let rhs_full = if matches!(self.peek(), Some(Tok::Op('^'))) {
                self.parse_pow_rhs(rhs_unary)?
            } else {
                rhs_unary
            };
            Ok(lhs.powf(rhs_full))
        } else {
            Ok(lhs)
        }
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
            _ => self.parse_atom(),
        }
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
        match lname.as_str() {
            "pi" => Ok(std::f64::consts::PI),
            "e" => Ok(std::f64::consts::E),
            _ => {
                // function?
                match self.peek() {
                    Some(Tok::LParen) => {
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
                        apply_func(&lname, &args)
                    }
                    _ => Err(ConvertError::Expr {
                        expr: id.clone(),
                        detail: format!("unknown ident {id}"),
                    }),
                }
            }
        }
    }
}

fn apply_func(name: &str, args: &[f64]) -> Result<f64> {
    let f = |n: &str| ConvertError::Expr {
        expr: n.into(),
        detail: "bad args".into(),
    };
    match name {
        "round" => {
            if args.len() != 1 {
                return Err(f(name));
            }
            Ok(args[0].round())
        }
        "floor" => {
            if args.len() != 1 {
                return Err(f(name));
            }
            Ok(args[0].floor())
        }
        "ceil" => {
            if args.len() != 1 {
                return Err(f(name));
            }
            Ok(args[0].ceil())
        }
        "abs" => {
            if args.len() != 1 {
                return Err(f(name));
            }
            Ok(args[0].abs())
        }
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
    fn pow() {
        assert_eq!(
            evaluate_expression("2^3^2", ExprNumberFormat::Float(2)).unwrap(),
            "512"
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
