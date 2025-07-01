use super::*;

primitive_aliases! { pub Usize => usize }

impl Scalar for usize {
    type Vec2Alignment = Align<{ size_of::<Self>() * 2 }>;
    type Vec3Alignment = Align<{ size_of::<Self>() * 4 }>;
    type Vec4Alignment = Align<{ size_of::<Self>() * 4 }>;
}
