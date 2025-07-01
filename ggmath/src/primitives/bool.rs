use super::*;

primitive_aliases! { pub B => bool }

impl Scalar for bool {
    type Vec2Alignment = Align<1>;
    type Vec3Alignment = Align<1>;
    type Vec4Alignment = Align<1>;
}
