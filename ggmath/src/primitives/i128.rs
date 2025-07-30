use super::*;

primitive_aliases! { pub I128 => i128 }

impl Scalar for i128 {
    type Vec2Alignment = Align<32>;
    type Vec3Alignment = Align<32>;
    type Vec4Alignment = Align<32>;

    // All bit patterns are valid ints, and no pattern can break operators.
    const PADDING: ScalarPadding<Self> = ScalarPadding::Uninit;
}

// i128 methods are defined using `macro_loop` in i8.rs
