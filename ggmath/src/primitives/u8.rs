use super::*;

primitive_aliases! { pub U8 => u8 }

impl Scalar for u8 {
    type Vec2Alignment = Align<2>;
    type Vec3Alignment = Align<4>;
    type Vec4Alignment = Align<4>;
}
