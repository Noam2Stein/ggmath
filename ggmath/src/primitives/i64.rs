use super::*;

primitive_aliases! { pub I64 => i64 }

impl Scalar for i64 {
    type Vec2Alignment = Align<16>;
    type Vec3Alignment = Align<32>;
    type Vec4Alignment = Align<32>;
}

// i64 methods are defined using `macro_loop` in i8.rs
