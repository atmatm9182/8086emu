use std::str::FromStr;
use std::sync::Arc;

use druid::{Lens, Data};

use crate::register::Register;
use crate::token::Token;

#[derive(Default, Clone, Data, Debug, Lens)]
pub struct Parser {
    pub contents: Arc<Vec<u8>>,
    pos: usize,
    peek_pos: usize,
}

impl Parser {
    pub fn new(s: impl Into<String>) -> Self {
        let contents = Arc::new(s.into().into_bytes());

        Self {
            contents,
            pos: 0,
            peek_pos: 0,
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.contents.get(self.pos) {
            if (*c as char).is_whitespace() {
                self.pos = self.peek_pos;
                self.peek_pos += 1;
            } else {
                break;
            }
        }
    }

    pub fn read_literal(&mut self) -> String {
        let p = self.pos;
        while let Some(c) = self.contents.get(self.pos) {
            if !(*c as char).is_alphabetic() {
                break;
            }

            self.pos = self.peek_pos;
            self.peek_pos += 1;
        }

        let s = String::from_utf8(self.contents[p..self.pos].to_vec()).unwrap();

        self.peek_pos -= 1;

        s
    }

    pub fn read_int(&mut self) -> String {
        let p = self.pos;
        while let Some(c) = self.contents.get(self.pos) {
            if !(*c as char).is_numeric() {
                break;
            }

            self.pos = self.peek_pos;
            self.peek_pos += 1;
        }

        let s = String::from_utf8(self.contents[p..self.pos].to_vec()).unwrap();

        self.peek_pos -= 1;

        s
    }
}

fn is_instruction(s: &str) -> bool {
    match s.to_lowercase().as_str() {
        "add" | "sub" | "mul" | "xchg" | "mov" | "div" | "and" | "or" | "xor" | "inc" | "dec"
        | "not" => true,
        _ => false,
    }
}

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        self.pos = self.peek_pos;
        self.peek_pos += 1;

        self.skip_whitespace();

        match self.contents.get(self.pos) {
            Some(c) => match *c as char {
                ',' => Some(Comma),
                c if c.is_numeric() => {
                    let lit = self.read_int();

                    let v: u8 = match lit.parse() {
                        Ok(v) => v,
                        Err(e) => match e.kind() {
                            std::num::IntErrorKind::PosOverflow => 255,
                            std::num::IntErrorKind::NegOverflow => 0,
                            _ => unreachable!()
                        }
                    };

                    Some(Num(v))
                }
                c if c.is_alphabetic() => {
                    let lit = self.read_literal();

                    if let Ok(_) = Register::from_str(&lit) {
                        return Some(Reg(lit));
                    } else if is_instruction(&lit) {
                        let left = self.next();
                        if let Some(left) = left {
                            match lit.to_lowercase().as_str() {
                                "inc" | "dec" | "not" => Some(Instruction {
                                    name: lit.clone(),
                                    left: Box::new(left),
                                    right: None,
                                }),
                                _ => {
                                    match self.next() {
                                        Some(Comma) => {}
                                        Some(_) => return Some(Illegal),
                                        None => return None,
                                    }
                                    let right = self.next();

                                    if let Some(right) = right {
                                        Some(Instruction {
                                            name: lit.clone(),
                                            left: Box::new(left),
                                            right: Some(Box::new(right)),
                                        })
                                    } else {
                                        None
                                    }
                                }
                            }
                        } else {
                            None
                        }
                        // this should be a comma
                    } else {
                        Some(Illegal)
                    }
                }
                _ => Some(Illegal),
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token::*;

    use super::Parser;

    #[test]
    fn pos() {
        let mut p = Parser::new("AH 123 AL add DH, 199 l \n inc CH".to_string());

        assert_eq!(Some(Reg("AH".into())), p.next());
        assert_eq!(Some(Num(123)), p.next());
        assert_eq!(Some(Reg("AL".into())), p.next());
        assert_eq!(
            Some(Instruction {
                name: "add".into(),
                left: Box::new(Reg("DH".into())),
                right: Some(Box::new(Num(199)))
            }),
            p.next()
        );
        assert_eq!(Some(Illegal), p.next());
        assert_eq!(
            Some(Instruction {
                name: "inc".into(),
                left: Box::new(Reg("CH".into())),
                right: None
            }),
            p.next()
        );
        assert_eq!(None, p.next());
    }
}
