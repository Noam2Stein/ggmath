use super::*;

primitive_aliases! { pub U16 => u16 }

impl Scalar for u16 {
    type Vec2Alignment = Align<4>;
    type Vec3Alignment = Align<8>;
    type Vec4Alignment = Align<8>;
}

// u16 methods are defined using `macro_loop` in other files
