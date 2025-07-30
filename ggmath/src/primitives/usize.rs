use super::*;

primitive_aliases! { pub Usize => usize }

impl Scalar for usize {
    type Vec2Alignment = Align<{ size_of::<Self>() * 2 }>;
    type Vec3Alignment = Align<{ size_of::<Self>() * 4 }>;
    type Vec4Alignment = Align<{ size_of::<Self>() * 4 }>;

    // All bit patterns are valid ints, and no pattern can break operators.
    const PADDING: ScalarPadding<Self> = ScalarPadding::Uninit;
}

// usize methods are defined using `macro_loop` in other files
