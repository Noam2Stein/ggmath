use std::mem::MaybeUninit;

use super::*;

primitive_aliases! { pub I8 => i8 }

impl Scalar for i8 {
    type Vec2Alignment = Align<2>;
    type Vec3Alignment = Align<4>;
    type Vec4Alignment = Align<4>;

    const NEG_GARBAGE: Option<fn(MaybeUninit<Self>) -> MaybeUninit<Self>> = Some(|x| unsafe {
        let x = x.assume_init();

        let output = x.checked_neg().unwrap_unchecked();

        MaybeUninit::new(output)
    });

    const NOT_GARBAGE: Option<fn(MaybeUninit<Self>) -> MaybeUninit<Self>> = Some(|x| unsafe {
        let x = x.assume_init();

        let output = !x;

        MaybeUninit::new(output)
    });

    const ADD_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x.checked_add(y).unwrap_unchecked();

            MaybeUninit::new(output)
        });

    const SUB_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x.checked_sub(y).unwrap_unchecked();

            MaybeUninit::new(output)
        });

    const MUL_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x.checked_mul(y).unwrap_unchecked();

            MaybeUninit::new(output)
        });

    const REM_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x.checked_rem(y).unwrap_unchecked();

            MaybeUninit::new(output)
        });

    const BITAND_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x & y;

            MaybeUninit::new(output)
        });

    const BITOR_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x | y;

            MaybeUninit::new(output)
        });

    const BITXOR_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x ^ y;

            MaybeUninit::new(output)
        });
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
