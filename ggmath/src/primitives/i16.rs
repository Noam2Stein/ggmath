use super::*;

primitive_aliases! { pub I16 => i16 }

impl Scalar for i16 {
    type Vec2Alignment = Align<4>;
    type Vec3Alignment = Align<8>;
    type Vec4Alignment = Align<8>;

    // All bit patterns are valid ints, and no pattern can break operators.
    const PADDING: ScalarPadding<Self> = ScalarPadding::Uninit;
}

// i16 methods are defined using `macro_loop` in i8.rs
