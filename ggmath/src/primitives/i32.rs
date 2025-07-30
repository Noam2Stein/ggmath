use super::*;

primitive_aliases! { pub I => i32 }

impl Scalar for i32 {
    type Vec2Alignment = Align<8>;
    type Vec3Alignment = Align<16>;
    type Vec4Alignment = Align<16>;

    // All bit patterns are valid ints, and no pattern can break operators.
    const PADDING: ScalarPadding<Self> = ScalarPadding::Uninit;
}

// i32 methods are defined using `macro_loop` in i8.rs
