use super::*;

primitive_aliases! { pub U128 => u128 }

impl Scalar for u128 {
    type Vec2Alignment = Align<32>;
    type Vec3Alignment = Align<32>;
    type Vec4Alignment = Align<32>;

    // All bit patterns are valid ints, and no pattern can break operators.
    const PADDING: ScalarPadding<Self> = ScalarPadding::Uninit;
}

// u128 methods are defined using `macro_loop` in other files
