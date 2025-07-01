use super::*;

primitive_aliases! { pub I8 => i8 }

impl Scalar for i8 {
    type Vec2Alignment = Align<2>;
    type Vec3Alignment = Align<4>;
    type Vec4Alignment = Align<4>;
}
