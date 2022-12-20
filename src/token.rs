#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Reg(String),
    Comma,
    Instruction {
        name: String,
        left: Box<Token>,
        right: Option<Box<Token>>,
    },
    Num(u8),
    Illegal,
}
