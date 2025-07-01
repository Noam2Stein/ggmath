use super::*;

primitive_aliases! { pub F => f32 }

impl Scalar for f32 {
    type Vec2Alignment = Align<8>;
    type Vec3Alignment = Align<16>;
    type Vec4Alignment = Align<16>;
}
