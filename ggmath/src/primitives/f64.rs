use super::*;

primitive_aliases! { pub D => f64 }

impl Scalar for f64 {
    type Vec2Alignment = Align<16>;
    type Vec3Alignment = Align<32>;
    type Vec4Alignment = Align<32>;

    const NEG_GARBAGE: Option<fn(Self) -> Self> = Some(|a| -a);
    const ADD_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a + b);
    const SUB_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a - b);
    const MUL_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a * b);
    const DIV_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a / b);
    const REM_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a % b);
}

// f64 methods are defined using `macro_loop` in other files
