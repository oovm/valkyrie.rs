use super::*;
use OperatorAssociativity::*;
use OperatorKind::*;

#[derive(Clone, Debug)]
pub enum OperatorKind {
    Prefix,
    Infix,
    Suffix,
}

#[derive(Clone, Debug)]
pub enum OperatorAssociativity {
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Operator {
    kind: OperatorKind,
    assoc: OperatorAssociativity,
    prec: u8,
    op: &'static str,
}

impl Operator {
    pub fn parse(o: &str, a: i8) -> Self {
        match a {
            a if a < 0 => Operator::parse_prefix(o),
            a if a > 0 => Operator::parse_suffix(o),
            _ => Operator::parse_infix(o),
        }
    }
    fn parse_prefix(o: &str) -> Self {
        match o {
            "+"  => Self::PREFIX_PLUS,
            "-"  => Self::PREFIX_MINUS,
            "!"  => Self::PREFIX_NOT,
            _ => unimplemented!("{:?}", o),
        }
    }
    fn parse_infix(o: &str) -> Self {
        match o {
            "+" => Self::INFIX_PLUS,
            "-" => Self::INFIX_MINUS,
            _ => unimplemented!("{:?}", o),
        }
    }
    fn parse_suffix(o: &str) -> Self {
        match o {
            "+"  => Self::SUFFIX_PLUS,
            "-"  => Self::SUFFIX_MINUS,
            "!"  => Self::SUFFIX_POWER,
            _ => unimplemented!("{:?}", o),
        }
    }
}

impl Operator {
    pub const PREFIX_PLUS: Self = Self { kind: Infix, assoc: Left, prec: 100, op: "+" };
    pub const PREFIX_MINUS: Self = Self { kind: Infix, assoc: Left, prec: 100, op: "-" };
    pub const PREFIX_NOT: Self = Self { kind: Infix, assoc: Right, prec: 120, op: "!" };
}


impl Operator {
    pub const INFIX_PLUS: Self = Self { kind: Infix, assoc: Left, prec: 100, op: "+" };
    pub const INFIX_MINUS: Self = Self { kind: Infix, assoc: Left, prec: 100, op: "-" };
    pub const INFIX_POWER: Self = Self { kind: Infix, assoc: Right, prec: 120, op: "*" };
}

impl Operator {
    pub const SUFFIX_PLUS: Self = Self { kind: Infix, assoc: Left, prec: 100, op: "+" };
    pub const SUFFIX_MINUS: Self = Self { kind: Infix, assoc: Left, prec: 100, op: "-" };
    pub const SUFFIX_POWER: Self = Self { kind: Infix, assoc: Right, prec: 120, op: "*" };
}
