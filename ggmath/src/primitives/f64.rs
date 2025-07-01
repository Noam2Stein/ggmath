use super::*;

primitive_aliases! { pub D => f64 }

impl Scalar for f64 {
    type Vec2Alignment = Align<16>;
    type Vec3Alignment = Align<32>;
    type Vec4Alignment = Align<32>;
}
