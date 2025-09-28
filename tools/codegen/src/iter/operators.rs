use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl UnOp {
    pub fn lowercase_str(&self) -> &'static str {
        match self {
            UnOp::Neg => "neg",
            UnOp::Not => "not",
        }
    }

    pub fn camelcase_str(&self) -> &'static str {
        match self {
            UnOp::Neg => "Neg",
            UnOp::Not => "Not",
        }
    }
}

impl BinOp {
    pub fn lowercase_str(&self) -> &'static str {
        match self {
            BinOp::Add => "add",
            BinOp::Sub => "sub",
            BinOp::Mul => "mul",
            BinOp::Div => "div",
            BinOp::Rem => "rem",
            BinOp::Shl => "shl",
            BinOp::Shr => "shr",
            BinOp::BitAnd => "bitand",
            BinOp::BitOr => "bitor",
            BinOp::BitXor => "bitxor",
        }
    }

    pub fn camelcase_str(&self) -> &'static str {
        match self {
            BinOp::Add => "Add",
            BinOp::Sub => "Sub",
            BinOp::Mul => "Mul",
            BinOp::Div => "Div",
            BinOp::Rem => "Rem",
            BinOp::Shl => "Shl",
            BinOp::Shr => "Shr",
            BinOp::BitAnd => "BitAnd",
            BinOp::BitOr => "BitOr",
            BinOp::BitXor => "BitXor",
        }
    }
}

impl CmpOp {
    pub fn lowercase_str(&self) -> &'static str {
        match self {
            CmpOp::Eq => "eq",
            CmpOp::Ne => "ne",
            CmpOp::Lt => "lt",
            CmpOp::Le => "le",
            CmpOp::Gt => "gt",
            CmpOp::Ge => "ge",
        }
    }

    pub fn trait_str(&self) -> &'static str {
        match self {
            CmpOp::Eq | CmpOp::Ne => "PartialEq",
            CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => "PartialOrd",
        }
    }

    pub fn punct_str(&self) -> &'static str {
        match self {
            CmpOp::Eq => "==",
            CmpOp::Ne => "!=",
            CmpOp::Lt => "<",
            CmpOp::Le => "<=",
            CmpOp::Gt => ">",
            CmpOp::Ge => ">=",
        }
    }

    pub fn doc_str(&self) -> &'static str {
        match self {
            CmpOp::Eq => "equal to",
            CmpOp::Ne => "not equal to",
            CmpOp::Lt => "less than",
            CmpOp::Le => "less than or equal to",
            CmpOp::Gt => "greater than",
            CmpOp::Ge => "greater than or equal to",
        }
    }
}
