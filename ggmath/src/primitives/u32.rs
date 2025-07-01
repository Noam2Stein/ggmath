use super::*;

primitive_aliases! { pub U => u32 }

impl Scalar for u32 {
    type Vec2Alignment = Align<8>;
    type Vec3Alignment = Align<16>;
    type Vec4Alignment = Align<16>;
}
