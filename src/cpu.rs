use std::str::FromStr;

use druid::{Lens, Data};

use crate::{
    parser::Parser,
    register::{Register},
    token::Token,
};

#[derive(Data, Lens, Debug, Clone)]
pub struct Cpu {
    pub registers: [Register; 8],
    pub parser: Parser,
}

impl Cpu {
    pub fn new(p: Parser) -> Self {
        Self {
            parser: p,
            ..Self::default()
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        let mut registers = [Register::AH(0); 8];

        let strs = ["AH", "AL", "BH", "BL", "CH", "CL", "DH", "DL"];

        for i in 0..=7 {
            registers[i] = Register::from_str(strs[i]).unwrap();
        }

        Cpu {
            registers,
            parser: Parser::default(),
        }
    }
}

impl Cpu {
    pub fn from_str(s: impl Into<String>) -> Self {
        Cpu {
            parser: Parser::new(s),
            ..Cpu::default()
        }
    }

    pub fn get_reg(&self, name: &str) -> Register {
        let idx = match name {
            "AH" => 0,
            "AL" => 1,
            "BH" => 2,
            "BL" => 3,
            "CH" => 4,
            "CL" => 5,
            "DH" => 6,
            "DL" => 7,
            _ => panic!("unreachable"),
        };

        self.registers[idx]
    }

    pub fn set_reg(&mut self, name: &str, val: Register) -> Result<(), String> {
        let idx = match name {
            "AH" => 0,
            "AL" => 1,
            "BH" => 2,
            "BL" => 3,
            "CH" => 4,
            "CL" => 5,
            "DH" => 6,
            "DL" => 7,
            _ => return Err("wrong name".to_string()),
        };

        self.registers[idx] = val;

        Ok(())
    }

    pub fn registers_str(&self) -> String {
        self.registers
            .iter()
            .enumerate()
            .fold("".into(), |s, (i, r)| {
                let str = if i == 0 { "" } else { "\n" };
                format!("{s}{str}{r}")
            })
    }

    pub fn print_registers(&self) {
        println!("{}", self.registers_str())
    }

    pub fn eval(&mut self, cmd: Token) -> Result<(), String> {
        match cmd {
            Token::Instruction { name, left, right } => {
                let left = match *left {
                    Token::Reg(n) => n,
                    _ => return Err("expected register name at the second position".to_string()),
                };

                let right_int = || match &right {
                    Some(v) => match &**v {
                        Token::Num(n) => Ok(n.clone()),
                        _ => Err("expected a number".to_string()),
                    },
                    None => Err("wrong number of arguments".to_string()),
                };

                let right_reg = || match &right {
                    Some(v) => match &**v {
                        Token::Reg(name) => Ok(name),
                        v => Err(
                            format!("expected a register at the second position {:?}", v)
                                .to_string(),
                        ),
                    },
                    None => Err("wrong number of arguments".to_string()),
                };

                match name.to_lowercase().as_str() {
                    "add" => {
                        if let Ok(i) = right_int() {
                            self.set_reg(&left, self.get_reg(&left) + i)?
                        } else {
                            let name = right_reg()?;
                            self.set_reg(&left, self.get_reg(&left) + self.get_reg(&name))?
                        }
                    }
                    "sub" => {
                        if let Ok(i) = right_int() {
                            self.set_reg(&left, self.get_reg(&left) - i)?
                        } else {
                            let name = right_reg()?;
                            self.set_reg(&left, self.get_reg(&left) - self.get_reg(&name))?
                        }
                    }
                    "mul" => {
                        if let Ok(i) = right_int() {
                            self.set_reg(&left, self.get_reg(&left) * i)?
                        } else {
                            let name = right_reg()?;
                            self.set_reg(&left, self.get_reg(&left) * self.get_reg(&name))?
                        }
                    }
                    "div" => {
                        if let Ok(i) = right_int() {
                            self.set_reg(&left, self.get_reg(&left) / i)?
                        } else {
                            let name = right_reg()?;
                            self.set_reg(&left, self.get_reg(&left) / self.get_reg(&name))?
                        }
                    }
                    "and" => {
                        if let Ok(i) = right_int() {
                            self.set_reg(&left, self.get_reg(&left) & i)?
                        } else {
                            let name = right_reg()?;
                            self.set_reg(&left, self.get_reg(&left) & self.get_reg(&name))?
                        }
                    }
                    "or" => {
                        if let Ok(i) = right_int() {
                            self.set_reg(&left, self.get_reg(&left) | i)?
                        } else {
                            let name = right_reg()?;
                            self.set_reg(&left, self.get_reg(&left) | self.get_reg(&name))?
                        }
                    }
                    "xor" => {
                        if let Ok(i) = right_int() {
                            self.set_reg(&left, self.get_reg(&left) ^ i)?
                        } else {
                            let name = right_reg()?;
                            self.set_reg(&left, self.get_reg(&left) ^ self.get_reg(&name))?
                        }
                    }
                    "not" => self.set_reg(&left, !self.get_reg(&left))?,
                    "inc" => self.set_reg(&left, self.get_reg(&left).inc())?,
                    "dec" => self.set_reg(&left, self.get_reg(&left).dec())?,
                    "xchg" => {
                        let left_reg = self.get_reg(&left);
                        let right = right_reg()?;
                        let right_reg = self.get_reg(&right);

                        self.set_reg(&left, self.get_reg(&left).set_value(right_reg.extract()))?;
                        self.set_reg(&right, self.get_reg(&right).set_value(left_reg.extract()))?;
                    }
                    "mov" => {
                        if let Ok(i) = right_int() {
                            self.set_reg(&left, self.get_reg(&left).set_value(i))?
                        } else {
                            let name = right_reg()?;
                            self.set_reg(&left, self.get_reg(&name))?
                        }
                    }
                    lol => {
                        println!("{lol}")
                    }
                }
                Ok(())
            }
            _ => Err("expected instruction".to_string()),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        while let Some(t) = self.parser.next() {
            self.eval(t)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::Cpu;
    use super::Register::*;

    #[test]
    fn cpu_default() {
        let cpu = Cpu::default();

        let regs = [AH(0), AL(0), BH(0), BL(0), CH(0), CL(0), DH(0), DL(0)];

        for (f, s) in regs.iter().zip(cpu.registers) {
            assert_eq!(f.to_owned(), s);
        }
    }

    #[test]
    fn cpu_instructions() {
        let str = r#"
            MOV AH, 10
        "#;

        let parser = Parser::new(str);
        let mut cpu = Cpu::new(parser);

        cpu.run().unwrap();

        cpu.print_registers();
    }
}
