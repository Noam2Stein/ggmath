use super::*;

primitive_aliases! { pub U => u32 }

impl Scalar for u32 {
    type Vec2Alignment = Align<8>;
    type Vec3Alignment = Align<16>;
    type Vec4Alignment = Align<16>;

    const NEG_GARBAGE: Option<fn(Self) -> Self> =
        Some(|a| unsafe { a.checked_neg().unwrap_unchecked() });

    const NOT_GARBAGE: Option<fn(Self) -> Self> = Some(|a| !a);

    const ADD_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_add(b).unwrap_unchecked() });

    const SUB_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_sub(b).unwrap_unchecked() });

    const MUL_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_mul(b).unwrap_unchecked() });

    const DIV_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_div(b).unwrap_unchecked() });

    const REM_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_rem(b).unwrap_unchecked() });

    const BITAND_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a & b);

    const BITOR_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a | b);

    const BITXOR_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a ^ b);
}

// u32 methods are defined using `macro_loop` in other files
