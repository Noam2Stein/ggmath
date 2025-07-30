use super::*;

primitive_aliases! { pub I8 => i8 }

impl Scalar for i8 {
    type Vec2Alignment = Align<2>;
    type Vec3Alignment = Align<4>;
    type Vec4Alignment = Align<4>;

    const NEG_GARBAGE: Option<fn(Self) -> Self> =
        Some(|a| unsafe { a.checked_neg().unwrap_unchecked() });

    const NOT_GARBAGE: Option<fn(Self) -> Self> = Some(|a| !a);

    const ADD_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_add(b).unwrap_unchecked() });

    const SUB_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_sub(b).unwrap_unchecked() });

    const MUL_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_mul(b).unwrap_unchecked() });

    const DIV_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_div(b).unwrap_unchecked() });

    const REM_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_rem(b).unwrap_unchecked() });

    const BITAND_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a & b);

    const BITOR_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a | b);

    const BITXOR_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a ^ b);
}

// impl for all sint types
macro_loop! {
    @for int in [i8, i16, i32, i64, i128, isize] {
        impl<const N: usize, A: VecAlignment> Vector<N, @int, A>
        where
            Usize<N>: VecLen,
        {
            /// Vector of all `-1` values.
            pub const NEG_ONE: Self = Self::splat(-1);

            /// Returns a vector of boolean values, where each element is `true` if the corresponding element in the input vector is negative, and `false` otherwise.
            /// This is equivalent to `self < 0`.
            pub fn is_negative(&self) -> Vector<N, bool, A> {
                self.map(|x| x < 0)
            }

            /// Returns a vector of boolean values, where each element is `true` if the corresponding element in the input vector is non-negative, and `false` otherwise.
            /// This is equivalent to `self >= 0`.
            pub fn is_bin_positive(&self) -> Vector<N, bool, A> {
                self.map(|x| x >= 0)
            }

            /// Returns a vector of absolute values of the input vector.
            pub fn abs(self) -> Self {
                self.map(|x| x.abs())
            }
            /// Returns a vector of negative absolute values of the input vector.
            pub fn neg_abs(self) -> Self {
                self.map(|x| -x.abs())
            }

            /// Returns a vector mapping the signum of the elements.
            pub fn signumt(self) -> Self {
                self.map(|x| x.signum())
            }
            /// Returns a vector mapping the signum of the elements.
            /// This acts like float signum, returning `1` for `+0` and `-1` for `-0`.
            /// Of course ints don't have `+0` and `-0`, so `0` is treated as `+0`.
            pub fn bin_signum(self) -> Self {
                self.map(|x| if x >= 0 { 1 } else { -1 })
            }
        }
    }
}
