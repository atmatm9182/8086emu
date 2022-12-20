use std::str::FromStr;

use druid::Data;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Data)]
pub enum Register {
    AH(u8),
    AL(u8),
    BH(u8),
    BL(u8),
    CH(u8),
    CL(u8),
    DH(u8),
    DL(u8),
}

impl Register {
    pub fn set_value(&self, v: u8) -> Self {
        use Register::*;


        match self {
            AH(_) => AH(v),
            AL(_) => AL(v),
            BH(_) => BH(v),
            BL(_) => BL(v),
            CH(_) => CH(v),
            CL(_) => CL(v),
            DH(_) => DH(v),
            DL(_) => DL(v),
        }
    }

    pub fn extract(&self) -> u8 {
        use Register::*;

        match self {
            AH(v) | AL(v) | BH(v) | BL(v) | CH(v) | CL(v) | DH(v) | DL(v) => *v,
        }
    }

    pub fn inc(&self) -> Self {
        let v = self.extract();
        let v = if v as usize + 1 > 255 { 255 } else { v + 1 };

        self.set_value(v)
    }

    pub fn dec(&self) -> Self {
        let v = self.extract();
        let v = if v as isize - 1 < 0 { 0 } else { v - 1 };

        self.set_value(v)
    }

    pub fn name(&self) -> &str {
        use Register::*;

        match self {
            AH(_) => "AH",
            AL(_) => "AL",
            BH(_) => "BH",
            BL(_) => "BL",
            CH(_) => "CH",
            CL(_) => "CL",
            DH(_) => "DH",
            DL(_) => "DL",
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name(), self.extract())
    }
}

impl std::ops::Add for Register {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.set_value(u8::saturating_add(self.extract(), rhs.extract()))
    }
}

impl std::ops::Add<u8> for Register {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        self.set_value(u8::saturating_add(self.extract(), rhs))
    }
}

impl std::ops::Sub for Register {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.set_value(u8::saturating_sub(self.extract(), rhs.extract()))
    }
}

impl std::ops::Sub<u8> for Register {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        self.set_value(u8::saturating_sub(self.extract(), rhs))
    }
}

impl std::ops::Mul<u8> for Register {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        self.set_value(u8::saturating_mul(self.extract(), rhs))
    }
}

impl std::ops::Mul for Register {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.set_value(u8::saturating_mul(self.extract(), rhs.extract()))
    }
}

impl std::ops::Div for Register {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.set_value(u8::saturating_div(self.extract(), rhs.extract()))
    }
}

impl std::ops::Div<u8> for Register {
    type Output = Self;

    fn div(self, rhs: u8) -> Self::Output {
        self.set_value(u8::saturating_div(self.extract(), rhs))
    }
}

impl std::ops::BitAnd for Register {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.set_value(self.extract() & rhs.extract())
    }
}

impl std::ops::BitAnd<u8> for Register {
    type Output = Self;

    fn bitand(self, rhs: u8) -> Self::Output {
        self.set_value(self.extract() & rhs)
    }
}

impl std::ops::BitOr for Register {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.set_value(self.extract() | rhs.extract())
    }
}

impl std::ops::BitOr<u8> for Register {
    type Output = Self;

    fn bitor(self, rhs: u8) -> Self::Output {
        self.set_value(self.extract() | rhs)
    }
}

impl std::ops::BitXor for Register {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.set_value(self.extract() ^ rhs.extract())
    }
}

impl std::ops::BitXor<u8> for Register {
    type Output = Self;

    fn bitxor(self, rhs: u8) -> Self::Output {
        self.set_value(self.extract() ^ rhs)
    }
}

impl std::ops::Not for Register {
    type Output = Self;

    fn not(self) -> Self {
        self.set_value(!self.extract())
    }
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Register::*;

        match s {
            "AH" => Ok(AH(0)),
            "AL" => Ok(AL(0)),
            "BH" => Ok(BH(0)),
            "BL" => Ok(BL(0)),
            "CH" => Ok(CH(0)),
            "CL" => Ok(CL(0)),
            "DH" => Ok(DH(0)),
            "DL" => Ok(DL(0)),
            _ => Err("This String is not a register".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Register::{self, *};

    #[test]
    fn plus_operator() {
        let a = Register::AH(1);
        let b = AL(4);

        assert_eq!(a + b, AH(5));
        assert_eq!(a + 4, AH(5))
    }

    #[test]
    fn minus_operator() {
        let a = Register::AH(1);
        let b = AL(4);

        assert_eq!(b - a, AL(3));
        assert_eq!(b - 1, AL(3))
    }

    #[test]
    fn mul_operator() {
        let a = Register::AH(2);
        let b = AL(4);

        assert_eq!(b * a, AL(8));
        assert_eq!(b * 2, AL(8))
    }

    #[test]
    fn div_operator() {
        let a = Register::AH(2);
        let b = AL(4);

        assert_eq!(b / a, AL(2));
        assert_eq!(b / 2, AL(2))
    }
}
