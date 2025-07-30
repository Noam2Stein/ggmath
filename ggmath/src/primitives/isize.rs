use super::*;

primitive_aliases! { pub Isize => isize }

impl Scalar for isize {
    type Vec2Alignment = <isized as Scalar>::Vec2Alignment;
    type Vec3Alignment = <isized as Scalar>::Vec3Alignment;
    type Vec4Alignment = <isized as Scalar>::Vec4Alignment;

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

// isize methods are defined using `macro_loop` in i8.rs

#[cfg(target_pointer_width = "16")]
#[allow(non_camel_case_types)]
type isized = i16;

#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
type isized = i32;

#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
type isized = i64;
