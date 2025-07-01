use super::*;

primitive_aliases! { pub Isize => isize }

impl Scalar for isize {
    type Vec2Alignment = Align<{ size_of::<Self>() * 2 }>;
    type Vec3Alignment = Align<{ size_of::<Self>() * 4 }>;
    type Vec4Alignment = Align<{ size_of::<Self>() * 4 }>;
}
